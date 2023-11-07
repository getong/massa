//! Copyright (c) 2022 MASSA LABS <info@massa.net>
//! Bootstrap crate
//!
//! At start up, if now is after genesis timestamp,
//! the node will bootstrap from one of the provided bootstrap servers.
//!
//! On server side, the server will query consensus for the graph and the ledger,
//! execution for execution related data and network for the peer list.
//!

#![warn(missing_docs)]
#![warn(unused_crate_dependencies)]

use massa_consensus_exports::bootstrapable_graph::BootstrapableGraph;
use massa_final_state::FinalState;
use massa_protocol_exports::BootstrapPeers;
use parking_lot::RwLock;
use std::io::{self, ErrorKind};
use std::sync::Arc;
use std::time::{Duration, Instant};

mod bindings;
mod client;
mod error;
pub use error::BootstrapError;
mod listener;
mod messages;
mod server;
/// bootstrap settings (pub for external tools)
pub mod settings;
mod tools;
/// white/black list
pub mod white_black_list;

pub use client::{get_state, DefaultConnector};
pub use listener::BootstrapTcpListener;
pub use messages::{
    BootstrapClientMessage, BootstrapClientMessageDeserializer, BootstrapClientMessageSerializer,
    BootstrapServerMessage, BootstrapServerMessageDeserializer, BootstrapServerMessageSerializer,
};
pub use server::{start_bootstrap_server, BootstrapManager};
pub use settings::IpType;
pub use settings::{BootstrapConfig, BootstrapServerMessageDeserializerArgs};

#[cfg(test)]
pub(crate) mod tests;

/// a collection of the bootstrap state snapshots of all relevant modules
pub struct GlobalBootstrapState {
    /// state of the final state
    pub final_state: Arc<RwLock<FinalState>>,

    /// state of the consensus graph
    pub graph: Option<BootstrapableGraph>,

    /// list of network peers
    pub peers: Option<BootstrapPeers>,
}

impl GlobalBootstrapState {
    fn new(final_state: Arc<RwLock<FinalState>>) -> Self {
        Self {
            final_state,
            graph: None,
            peers: None,
        }
    }
}

trait BindingReadExact: io::Read {
    /// similar to std::io::Read::read_exact, but with a timeout that is function-global instead of per-individual-read
    fn read_exact_timeout(
        &mut self,
        buf: &mut [u8],
        deadline: Option<Instant>,
    ) -> Result<(), (std::io::Error, usize)> {
        let mut count = 0;
        self.set_read_timeout(None).map_err(|err| (err, count))?;
        while count < buf.len() {
            // update the timeout
            if let Some(deadline) = deadline {
                let dur = deadline.saturating_duration_since(Instant::now());
                if dur.is_zero() {
                    return Err((
                        std::io::Error::new(ErrorKind::TimedOut, "deadline has elapsed"),
                        count,
                    ));
                }
                self.set_read_timeout(Some(dur))
                    .map_err(|err| (err, count))?;
            }

            // do the read
            match self.read(&mut buf[count..]) {
                Ok(0) => break,
                Ok(n) => {
                    count += n;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err((e, count)),
            }
        }
        if count != buf.len() {
            Err((
                std::io::Error::new(ErrorKind::UnexpectedEof, "failed to fill whole buffer"),
                count,
            ))
        } else {
            Ok(())
        }
    }

    /// Internal helper
    fn set_read_timeout(&mut self, deadline: Option<Duration>) -> Result<(), std::io::Error>;
}
