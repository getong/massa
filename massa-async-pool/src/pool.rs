//! Copyright (c) 2022 MASSA LABS <info@massa.net>

//! This file defines a finite size final pool of asynchronous messages for use in the context of autonomous smart contracts

use crate::{
    changes::{AsyncPoolChanges, Change},
    config::AsyncPoolConfig,
    message::{AsyncMessage, AsyncMessageId},
    AsyncMessageDeserializer, AsyncMessageIdDeserializer, AsyncMessageIdSerializer,
    AsyncMessageSerializer, AsyncMessageTrigger,
};
use massa_hash::{Hash, HASH_SIZE_BYTES};
use massa_ledger_exports::{LedgerBatch, LedgerChanges};
use massa_models::{slot::Slot, streaming_step::StreamingStep};
use massa_serialization::{
    DeserializeError, Deserializer, SerializeError, Serializer, U64VarIntDeserializer,
    U64VarIntSerializer,
};
use nom::{
    error::{context, ContextError, ParseError},
    multi::length_count,
    sequence::tuple,
    IResult, Parser,
};
use parking_lot::RwLock;
use rocksdb::{Direction, IteratorMode, Options, ReadOptions, DB};
use std::ops::Bound::Included;
use std::{collections::BTreeMap, sync::Arc};

const ASYNC_POOL_HASH_INITIAL_BYTES: &[u8; 32] = &[0; HASH_SIZE_BYTES];
pub const ASYNC_POOL_CF: &str = "async_pool";
const CF_ERROR: &str = "critical: rocksdb column family operation failed";
//const OPEN_ERROR: &str = "critical: rocksdb open operation failed";
const CRUD_ERROR: &str = "critical: rocksdb crud operation failed";
const WRONG_BATCH_TYPE_ERROR: &str = "critical: wrong batch type";
const MESSAGE_DESER_ERROR: &str = "critical: message deserialization failed";
const MESSAGE_SER_ERROR: &str = "critical: message serialization failed";
const MESSAGE_ID_DESER_ERROR: &str = "critical: message_id deserialization failed";
const MESSAGE_ID_SER_ERROR: &str = "critical: message_id serialization failed";

const METADATA_CF: &str = "metadata";
const ASYNC_POOL_HASH_ERROR: &str = "critical: saved async pool hash is corrupted";
const ASYNC_POOL_HASH_KEY: &[u8; 1] = b"h";

#[derive(Clone)]
/// Represents a pool of sorted messages in a deterministic way.
/// The final asynchronous pool is attached to the output of the latest final slot within the context of massa-final-state.
/// Nodes must bootstrap the final message pool when they join the network.
pub struct AsyncPool {
    /// Asynchronous pool configuration
    config: AsyncPoolConfig,
    pub db: Arc<RwLock<DB>>,
    message_id_serializer: AsyncMessageIdSerializer,
    message_serializer: AsyncMessageSerializer,
    message_id_deserializer: AsyncMessageIdDeserializer,
    message_deserializer: AsyncMessageDeserializer,
}

impl AsyncPool {
    /// Creates an empty `AsyncPool`
    pub fn new(config: AsyncPoolConfig, db: Arc<RwLock<DB>>) -> AsyncPool {
        AsyncPool {
            config: config.clone(),
            db,
            message_id_serializer: AsyncMessageIdSerializer::new(),
            message_serializer: AsyncMessageSerializer::new(),
            message_id_deserializer: AsyncMessageIdDeserializer::new(config.thread_count),
            message_deserializer: AsyncMessageDeserializer::new(
                config.thread_count,
                config.max_async_message_data,
                config.max_key_length,
            ),
        }
    }

    /// Creates an `AsyncPool` from an existing snapshot (and recomputes the hash)
    pub fn from_snapshot(
        config: AsyncPoolConfig,
        messages: BTreeMap<AsyncMessageId, AsyncMessage>,
        db: Arc<RwLock<DB>>,
    ) -> AsyncPool {
        let mut pool = AsyncPool {
            config: config.clone(),
            db,
            message_id_serializer: AsyncMessageIdSerializer::new(),
            message_serializer: AsyncMessageSerializer::new(),
            message_id_deserializer: AsyncMessageIdDeserializer::new(config.thread_count),
            message_deserializer: AsyncMessageDeserializer::new(
                config.thread_count,
                config.max_async_message_data,
                config.max_key_length,
            ),
        };
        pool.set_pool_part(messages);
        pool
    }

    /// Resets the pool to its initial state
    ///
    /// USED ONLY FOR BOOTSTRAP
    pub fn reset(&mut self) {
        let mut db = self.db.write();
        (*db)
            .drop_cf(ASYNC_POOL_CF)
            .expect("Error dropping async_pool cf");
        let mut db_opts = Options::default();
        db_opts.set_error_if_exists(true);
        (*db)
            .create_cf(ASYNC_POOL_CF, &db_opts)
            .expect("Error creating async_pool cf");
    }

    /// Applies pre-compiled `AsyncPoolChanges` to the pool without checking for overflows.
    /// This function is used when applying pre-compiled `AsyncPoolChanges` to an `AsyncPool`.
    ///
    /// # arguments
    /// * `changes`: `AsyncPoolChanges` listing all asynchronous pool changes (message insertions/deletions)
    pub fn apply_changes_unchecked_to_batch(
        &self,
        changes: &AsyncPoolChanges,
        batch: &mut LedgerBatch,
    ) {
        for change in changes.0.iter() {
            match change {
                // add a new message to the pool
                Change::Add(message_id, message) => {
                    self.put_entry(message_id, message.clone(), batch);
                }

                Change::Activate(message_id) => {
                    self.activate_entry(message_id, batch);
                }

                // delete a message from the pool
                Change::Delete(message_id) => {
                    self.delete_entry(message_id, batch);
                }
            }
        }
    }

    /// Settles a slot, adding new messages to the pool and returning expired and excess ones.
    /// This method is called at the end of a slot execution to apply the list of emitted messages,
    /// and get the list of pruned messages for `coins` reimbursement.
    ///
    /// # arguments
    /// * `slot`: used to filter out expired messages, not stored
    /// * `new_messages`: list of `AsyncMessage` to add to the pool
    ///
    /// # returns
    /// The list of `(message_id, message)` that were eliminated from the pool after the changes were applied, sorted in the following order:
    /// * expired messages from the pool, in priority order (from highest to lowest priority)
    /// * expired messages from `new_messages` (in the order they appear in `new_messages`)
    /// * excess messages after inserting all remaining `new_messages`, in priority order (from highest to lowest priority)
    /// The list of message that their trigger has been triggered.
    #[allow(clippy::type_complexity)]
    pub fn settle_slot(
        &self,
        slot: &Slot,
        new_messages: &mut Vec<(AsyncMessageId, AsyncMessage)>,
        ledger_changes: &LedgerChanges,
    ) -> (
        Vec<(AsyncMessageId, AsyncMessage)>,
        Vec<(AsyncMessageId, AsyncMessage)>,
    ) {
        let mut batch = LedgerBatch::new(None, Some(self.get_hash()));
        let mut eliminated = Vec::new();

        let db = self.db.read();
        let handle = db.cf_handle(ASYNC_POOL_CF).expect(CF_ERROR);

        for (serialized_message_id, serialized_message) in
            db.iterator_cf(handle, IteratorMode::Start).flatten()
        {
            let (_, message) = self
                .message_deserializer
                .deserialize::<DeserializeError>(&serialized_message)
                .expect(MESSAGE_DESER_ERROR);
            if *slot >= message.validity_end {
                let (_, message_id) = self
                    .message_id_deserializer
                    .deserialize::<DeserializeError>(&serialized_message_id)
                    .expect(MESSAGE_ID_DESER_ERROR);
                eliminated.push((message_id, message));
                self.delete_entry(&message_id, &mut batch)
            }
        }

        // Filter out all messages for which the validity end is expired.
        // Note that the validity_end bound is NOT included in the validity interval of the message.
        eliminated.extend(new_messages.drain_filter(|(_k, v)| *slot >= v.validity_end));

        // Insert new messages into the pool
        for (message_id, message) in new_messages.iter() {
            self.put_entry(message_id, message.clone(), &mut batch);
        }

        // Truncate message pool to its max size, removing non-prioritary items
        let excess_count = db
            .iterator_cf(handle, IteratorMode::Start)
            .count()
            .saturating_sub(self.config.max_length as usize);
        eliminated.reserve_exact(excess_count);

        for _ in 0..excess_count {
            let (serialized_message_id, serialized_message) = db
                .iterator_cf(handle, IteratorMode::End)
                .next()
                .unwrap()
                .unwrap(); // will not panic (checked at excess_count computation)

            let (_, message_id) = self
                .message_id_deserializer
                .deserialize::<DeserializeError>(&serialized_message_id)
                .expect(MESSAGE_ID_DESER_ERROR);
            let (_, message) = self
                .message_deserializer
                .deserialize::<DeserializeError>(&serialized_message)
                .expect(MESSAGE_DESER_ERROR);

            eliminated.push((message_id, message)); // will not panic (checked at excess_count computation)
            self.delete_entry(&message_id, &mut batch);
        }

        let mut triggered = Vec::new();

        for (serialized_message_id, serialized_message) in
            db.iterator_cf(handle, IteratorMode::Start).flatten()
        {
            let (_, mut message) = self
                .message_deserializer
                .deserialize::<DeserializeError>(&serialized_message)
                .expect(MESSAGE_DESER_ERROR);

            if let Some(filter) = &message.trigger && !message.can_be_executed && is_triggered(filter, ledger_changes) {
                let (_, message_id) = self
                    .message_id_deserializer
                    .deserialize::<DeserializeError>(&serialized_message_id)
                    .expect(MESSAGE_ID_DESER_ERROR);

                message.can_be_executed = true;
                triggered.push((message_id, message.clone()));

            }
        }

        (eliminated, triggered)
    }

    /// Takes the best possible batch of messages to execute, with gas limits and slot validity filtering.
    /// The returned messages are removed from the pool.
    /// This method is used at the beginning of a slot execution to list asynchronous messages to execute.
    ///
    /// # arguments
    /// * `slot`: select only messages that are valid within this slot
    /// * `available_gas`: maximum amount of available gas
    ///
    /// # returns
    /// A vector of messages, sorted from the most priority to the least priority
    pub fn take_batch_to_execute(
        &mut self,
        slot: Slot,
        mut available_gas: u64,
    ) -> Vec<(AsyncMessageId, AsyncMessage)> {
        // gather all selected items and remove them from self.messages
        // iterate in decreasing priority order
        let mut batch = LedgerBatch::new(None, Some(self.get_hash()));
        let mut taken = Vec::new();

        let db = self.db.read();
        let handle = db.cf_handle(ASYNC_POOL_CF).expect(CF_ERROR);

        for (serialized_message_id, serialized_message) in
            db.iterator_cf(handle, IteratorMode::Start).flatten()
        {
            let (_, message) = self
                .message_deserializer
                .deserialize::<DeserializeError>(&serialized_message)
                .expect(MESSAGE_DESER_ERROR);

            if available_gas >= message.max_gas
                && slot >= message.validity_start
                && slot < message.validity_end
                && message.can_be_executed
            {
                available_gas -= message.max_gas;
                let (_, message_id) = self
                    .message_id_deserializer
                    .deserialize::<DeserializeError>(&serialized_message_id)
                    .expect(MESSAGE_ID_DESER_ERROR);
                taken.push((message_id, message));
                self.delete_entry(&message_id, &mut batch);
            }
        }

        self.write_batch(batch);

        taken
    }

    /// Get a part of the async pool.
    /// Used for bootstrap.
    ///
    /// # Arguments
    /// * cursor: current bootstrap state
    ///
    /// # Returns
    /// The async pool part and the updated cursor
    pub fn get_pool_part(
        &self,
        cursor: StreamingStep<AsyncMessageId>,
    ) -> (
        BTreeMap<AsyncMessageId, AsyncMessage>,
        StreamingStep<AsyncMessageId>,
    ) {
        let db = self.db.read();
        let handle = db.cf_handle(ASYNC_POOL_CF).expect(CF_ERROR);
        let opt = ReadOptions::default();

        let mut pool_part = BTreeMap::new();
        // Creates an iterator from the next element after the last if defined, otherwise initialize it at the first key of the ledger.
        let (db_iterator, mut new_cursor) = match cursor {
            StreamingStep::Started => (
                db.iterator_cf_opt(handle, opt, IteratorMode::Start),
                StreamingStep::<AsyncMessageId>::Started,
            ),
            StreamingStep::Ongoing(last_id) => {
                let mut serialized_message_id = Vec::new();
                self.message_id_serializer
                    .serialize(&last_id, &mut serialized_message_id)
                    .expect(MESSAGE_ID_SER_ERROR);
                let mut iter = db.iterator_cf_opt(
                    handle,
                    opt,
                    IteratorMode::From(&serialized_message_id, Direction::Forward),
                );
                iter.next();
                (iter, StreamingStep::Finished(None))
            }
            StreamingStep::<AsyncMessageId>::Finished(_) => return (pool_part, cursor),
        };

        // Iterates over the whole database
        for (serialized_message_id, serialized_message) in db_iterator.flatten() {
            if pool_part.len() < self.config.bootstrap_part_size as usize {
                let (_, message_id) = self
                    .message_id_deserializer
                    .deserialize::<DeserializeError>(&serialized_message_id)
                    .expect("MESSAGE_ID_DESER_ERROR");
                let (_, message) = self
                    .message_deserializer
                    .deserialize::<DeserializeError>(&serialized_message)
                    .expect(MESSAGE_DESER_ERROR);
                pool_part.insert(message_id, message.clone());

                new_cursor = StreamingStep::Ongoing(message_id);
            } else {
                break;
            }
        }
        (pool_part, new_cursor)
    }

    /// Set a part of the async pool.
    /// Used for bootstrap.
    ///
    /// # Arguments
    /// * part: the async pool part provided by `get_pool_part`
    ///
    /// # Returns
    /// The updated cursor after the current insert
    pub fn set_pool_part(
        &mut self,
        part: BTreeMap<AsyncMessageId, AsyncMessage>,
    ) -> StreamingStep<AsyncMessageId> {
        let mut batch = LedgerBatch::new(None, Some(self.get_hash()));

        let cursor = if let Some(message_id) = part.last_key_value().map(|(&id, _)| id) {
            StreamingStep::Ongoing(message_id)
        } else {
            StreamingStep::Finished(None)
        };

        for (message_id, message) in part {
            self.put_entry(&message_id, message, &mut batch);
        }

        self.write_batch(batch);

        cursor
    }
}

/// Check in the ledger changes if a message trigger has been triggered
fn is_triggered(filter: &AsyncMessageTrigger, ledger_changes: &LedgerChanges) -> bool {
    ledger_changes.has_changes(&filter.address, filter.datastore_key.clone())
}

/// Serializer for `AsyncPool`
pub struct AsyncPoolSerializer {
    u64_serializer: U64VarIntSerializer,
    async_message_id_serializer: AsyncMessageIdSerializer,
    async_message_serializer: AsyncMessageSerializer,
}

impl Default for AsyncPoolSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl AsyncPoolSerializer {
    /// Creates a new `AsyncPool` serializer
    pub fn new() -> Self {
        Self {
            u64_serializer: U64VarIntSerializer::new(),
            async_message_id_serializer: AsyncMessageIdSerializer::new(),
            async_message_serializer: AsyncMessageSerializer::new(),
        }
    }
}

impl Serializer<BTreeMap<AsyncMessageId, AsyncMessage>> for AsyncPoolSerializer {
    fn serialize(
        &self,
        value: &BTreeMap<AsyncMessageId, AsyncMessage>,
        buffer: &mut Vec<u8>,
    ) -> Result<(), SerializeError> {
        // async pool length
        self.u64_serializer
            .serialize(&(value.len() as u64), buffer)?;
        // async pool
        for (message_id, message) in value {
            self.async_message_id_serializer
                .serialize(message_id, buffer)?;
            self.async_message_serializer.serialize(message, buffer)?;
        }
        Ok(())
    }
}

/// Deserializer for `AsyncPool`
pub struct AsyncPoolDeserializer {
    u64_deserializer: U64VarIntDeserializer,
    async_message_id_deserializer: AsyncMessageIdDeserializer,
    async_message_deserializer: AsyncMessageDeserializer,
}

impl AsyncPoolDeserializer {
    /// Creates a new `AsyncPool` deserializer
    pub fn new(
        thread_count: u8,
        max_async_pool_length: u64,
        max_async_message_data: u64,
        max_key_length: u32,
    ) -> AsyncPoolDeserializer {
        AsyncPoolDeserializer {
            u64_deserializer: U64VarIntDeserializer::new(
                Included(0),
                Included(max_async_pool_length),
            ),
            async_message_id_deserializer: AsyncMessageIdDeserializer::new(thread_count),
            async_message_deserializer: AsyncMessageDeserializer::new(
                thread_count,
                max_async_message_data,
                max_key_length,
            ),
        }
    }
}

impl Deserializer<BTreeMap<AsyncMessageId, AsyncMessage>> for AsyncPoolDeserializer {
    fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        &self,
        buffer: &'a [u8],
    ) -> IResult<&'a [u8], BTreeMap<AsyncMessageId, AsyncMessage>, E> {
        context(
            "Failed async_pool_part deserialization",
            length_count(
                context("Failed length deserialization", |input| {
                    self.u64_deserializer.deserialize(input)
                }),
                tuple((
                    context("Failed async_message_id deserialization", |input| {
                        self.async_message_id_deserializer.deserialize(input)
                    }),
                    context("Failed async_message deserialization", |input| {
                        self.async_message_deserializer.deserialize(input)
                    }),
                )),
            ),
        )
        .map(|vec| vec.into_iter().collect())
        .parse(buffer)
    }
}

#[test]
fn test_take_batch() {
    use massa_hash::Hash;
    use massa_ledger_worker::new_rocks_db_instance;
    use massa_models::{
        address::{Address, UserAddress},
        amount::Amount,
        slot::Slot,
    };
    use std::str::FromStr;
    use tempfile::TempDir;

    let config = AsyncPoolConfig {
        thread_count: 2,
        max_length: 10,
        max_async_message_data: 1_000_000,
        bootstrap_part_size: 100,
        max_key_length: 1000,
    };
    let tempdir = TempDir::new().expect("cannot create temp directory");
    let rocks_db_instance = Arc::new(RwLock::new(new_rocks_db_instance(
        tempdir.path().to_path_buf(),
    )));
    let mut pool = AsyncPool::new(config, rocks_db_instance.clone());
    let address = Address::User(UserAddress(Hash::compute_from(b"abc")));
    let mut batch = LedgerBatch::new(None, Some(pool.get_hash()));
    for i in 1..10 {
        let message = AsyncMessage::new_with_hash(
            Slot::new(0, 0),
            0,
            address,
            address,
            "function".to_string(),
            i,
            Amount::from_str("0.1").unwrap(),
            Amount::from_str("0.3").unwrap(),
            Slot::new(1, 0),
            Slot::new(3, 0),
            Vec::new(),
            None,
        );
        pool.put_entry(&message.compute_id(), message, &mut batch);
    }
    pool.write_batch(batch);
    let db = rocks_db_instance.read();
    let handle = db.cf_handle(ASYNC_POOL_CF).expect(CF_ERROR);
    assert_eq!(db.iterator_cf(handle, IteratorMode::Start).count(), 9);
    let a = pool.take_batch_to_execute(Slot::new(2, 0), 19);
    assert_eq!(a.len(), 5);
    assert_eq!(db.iterator_cf(handle, IteratorMode::Start).count(), 4);
}

// Private helpers
impl AsyncPool {
    /// Add every sub-entry individually for a given entry.
    ///
    /// # Arguments
    /// * `message_id`
    /// * `message`
    /// * `batch`: the given operation batch to update
    fn put_entry(
        &self,
        message_id: &AsyncMessageId,
        message: AsyncMessage,
        batch: &mut LedgerBatch,
    ) {
        let db = self.db.read();
        let handle = db.cf_handle(ASYNC_POOL_CF).expect(CF_ERROR);
        let mut serialized_message_id = Vec::new();
        self.message_id_serializer
            .serialize(message_id, &mut serialized_message_id)
            .expect(MESSAGE_ID_SER_ERROR);
        let mut serialized_message = Vec::new();
        self.message_serializer
            .serialize(&message, &mut serialized_message)
            .expect(MESSAGE_SER_ERROR);

        let hash = Hash::compute_from(
            &[serialized_message_id.clone(), serialized_message.clone()].concat(),
        );
        *batch
            .async_pool_hash
            .as_mut()
            .expect(WRONG_BATCH_TYPE_ERROR) ^= hash;
        batch.aeh_list.insert(serialized_message_id.clone(), hash);
        batch
            .write_batch
            .put_cf(handle, serialized_message_id, serialized_message);
    }

    /// Update the ledger entry of a given address.
    ///
    /// # Arguments
    /// * `entry_update`: a descriptor of the entry updates to be applied
    /// * `batch`: the given operation batch to update
    fn activate_entry(&self, message_id: &AsyncMessageId, batch: &mut LedgerBatch) {
        let db = self.db.read();

        let handle = db.cf_handle(ASYNC_POOL_CF).expect(CF_ERROR);
        let mut serialized_message_id = Vec::new();
        self.message_id_serializer
            .serialize(message_id, &mut serialized_message_id)
            .expect(MESSAGE_ID_SER_ERROR);

        if let Some(prev_bytes) = db.get_cf(handle, &serialized_message_id).expect(CRUD_ERROR) {
            *batch
                .async_pool_hash
                .as_mut()
                .expect(WRONG_BATCH_TYPE_ERROR) ^=
                Hash::compute_from(&[&serialized_message_id, &prev_bytes[..]].concat());

            let (_rest, mut message) = self
                .message_deserializer
                .deserialize::<DeserializeError>(&prev_bytes)
                .expect(MESSAGE_DESER_ERROR);
            message.can_be_executed = true;
            message.compute_hash();

            let mut serialized_message = Vec::new();
            self.message_serializer
                .serialize(&message, &mut serialized_message)
                .expect(MESSAGE_SER_ERROR);

            let hash = Hash::compute_from(
                &[serialized_message_id.clone(), serialized_message.clone()].concat(),
            );
            *batch
                .async_pool_hash
                .as_mut()
                .expect(WRONG_BATCH_TYPE_ERROR) ^= hash;
            batch.aeh_list.insert(serialized_message_id.clone(), hash);
            batch
                .write_batch
                .put_cf(handle, serialized_message_id, &serialized_message);
        }
    }

    /// Delete every sub-entry associated to the given address.
    ///
    /// # Arguments
    /// * batch: the given operation batch to update
    fn delete_entry(&self, message_id: &AsyncMessageId, batch: &mut LedgerBatch) {
        let db = self.db.read();
        let handle = db.cf_handle(ASYNC_POOL_CF).expect(CF_ERROR);
        let mut serialized_message_id = Vec::new();
        self.message_id_serializer
            .serialize(message_id, &mut serialized_message_id)
            .expect(MESSAGE_ID_SER_ERROR);

        if let Some(added_hash) = batch.aeh_list.get(&serialized_message_id) {
            *batch
                .async_pool_hash
                .as_mut()
                .expect(WRONG_BATCH_TYPE_ERROR) ^= *added_hash;
        } else if let Some(prev_bytes) = db
            .get_pinned_cf(handle, &serialized_message_id)
            .expect(CRUD_ERROR)
        {
            *batch
                .async_pool_hash
                .as_mut()
                .expect(WRONG_BATCH_TYPE_ERROR) ^=
                Hash::compute_from(&[&serialized_message_id, &prev_bytes[..]].concat());
        }
        batch.write_batch.delete_cf(handle, serialized_message_id);
    }

    /// Get the current async pool hash
    pub fn get_hash(&self) -> Hash {
        let db = self.db.read();
        let handle = db.cf_handle(METADATA_CF).expect(CF_ERROR);
        if let Some(async_pool_hash_bytes) = db
            .get_cf(handle, ASYNC_POOL_HASH_KEY)
            .expect(CRUD_ERROR)
            .as_deref()
        {
            Hash::from_bytes(
                async_pool_hash_bytes
                    .try_into()
                    .expect(ASYNC_POOL_HASH_ERROR),
            )
        } else {
            // initial async_hash value to avoid matching an option in every XOR operation
            // because of a one time case being an empty ledger
            // also note that the if you XOR a hash with itself result is LEDGER_HASH_INITIAL_BYTES
            Hash::from_bytes(ASYNC_POOL_HASH_INITIAL_BYTES)
        }
    }

    /// Apply the given operation batch to the disk ledger
    pub fn write_batch(&self, mut batch: LedgerBatch) {
        let db = self.db.read();
        let handle = db.cf_handle(METADATA_CF).expect(CF_ERROR);
        batch.write_batch.put_cf(
            handle,
            ASYNC_POOL_HASH_KEY,
            batch
                .async_pool_hash
                .expect(WRONG_BATCH_TYPE_ERROR)
                .to_bytes(),
        );
        db.write(batch.write_batch).expect(CRUD_ERROR);
    }
}