searchState.loadedDescShard("massa_execution_exports", 0, "Overview\ngets the balance (candidate) of an address, returns …\ngets the balance (final) of an address, returns …\ngets the bytecode (candidate) of an address, returns …\ngets the bytecode (final) of an address, returns …\ngets the datastore keys (candidate) of an address, returns …\ngets the datastore keys (final) of an address, returns …\ngets a datastore value (candidate) for an address, returns …\ngets a datastore value (final) for an address, returns …\ngets the deferred credits (candidate) of an address, …\ngets the deferred credits (final) of an address, returns …\nchecks if address exists (candidate) returns …\nchecks if address exists (final) returns …\ngets the roll count (candidate) of an address, returns …\ngets the roll count (final) of an address, returns …\nThe operation or denunciation was found as executed with …\nThe operation or denunciation was found as successfully …\namount value\nBlock gas error: {0}\nboolean value\nbytecode\nExecute the main function of a bytecode\nCache error: {0}\nChannel error\nget all information for a given cycle, returns …\ncycle infos value\ndatastore value\ndeferred credits value\ngets the execution status (candidate) for an denunciation, …\ngets the execution status (final) for an denunciation, …\nStore for events emitted by smart contracts\nget filtered events. Returns …\nEvents\nNo information about the operation or denunciation …\nstructure storing a block id + network versions (from a …\nExecuted slot output\nExecution info about an address\nMetadata needed to execute the block\nchannels used by the execution worker\nExecution module configuration\ninterface that communicates with the execution worker …\nErrors of the execution component.\nExecution manager used to stop the execution thread\nstructure describing the output of a single execution\nInformation about cycles\nExecution query errors\nExecution status of an operation or denunciation\nRequest to atomically execute a batch of execution state …\nExecution state query item\nResponse to a list of execution queries\nExecution state query response item\nStaker information for a given cycle\nStructure describing an element of the execution stack. …\nexecution status value\nFactory error: {0}\nFinalized slot output\nExecute a function call\nInclude denunciation error: {0}\nInclude operation error: {0}\nInvalid slot range\nlist of keys\n<code>MassaHashError</code>: {0}\n<code>ModelsError</code>: {0}\nNot enough gas in the block: {0}\nNot found: {0}\ngets the execution status (candidate) for an operation, …\ngets the execution status (final) for an operation, …\nstructure describing a read-only call\nstructure describing the output of a read only execution\nstructure describing different types of read-only …\nstructure describing different possible targets of a …\n<code>RollBuy</code> error: {0}\nroll counts value\n<code>RollSell</code> error: {0}\nRuntime error: {0}\nSlash roll or deferred credits  error: {0}\nstructure describing the output of the execution of a slot\nStorage cost constants\nGiven gas is above the threshold: {0}\n<code>Transaction</code> error: {0}\nVM Error in {context} context: {error}\nactive roll count\nCalled address\nAnnounced network version (see Versioning doc)\nconstant cost for async messages\nAuto sell roll execution (empty if execution-info feature …\nGas used by a transaction, a roll buy or a roll sell)\nWhere to dump blocks\nBlock id\noptional executed block info at that slot (None if miss)\nblock creation reward\nwhether slot execution outputs broadcast is enabled\nslot execution outputs channel capacity\nslot execution traces channel capacity\nwhether slot execution traces broadcast is enabled\nReturned value from the module call\nCall stack to simulate, older caller first\nCall stack to simulate, older caller first. Target should …\nCancel async message execution (empty if execution-info …\ncandidate balance of the address\nLast executed candidate slot\ncandidate datastore keys of the address\ncandidate number of rolls the address has\nchain id\nReturns a boxed clone of self. Useful to allow cloning …\nCoins transferred to the target address during the call\nCoins transferred to the target address during the call\nThis module exports generic traits representing interfaces …\nCurrent network version (see Versioning doc)\nextra lag to add on the execution cursor to improve …\ncycle number\ncycle information\nDeferred credits execution (empty if execution-info …\nDenunciation expire delta\nendorsement count\nthis file defines all possible execution error categories\nThis module represents an event store allowing to store, …\nevents emitted by the execution step\nExecute read-only SC function call without causing …\nFee\nfinal balance of the address\nLast executed final slot\nfinal datastore keys of the address\nfinal number of rolls the address has\nFinal state hash\nReturns the argument unchanged.\nfuture deferred credits\nGas cost for this execution, with needed adjustments\nGas costs\ngenesis timestamp\nGets information about a batch of addresses\nReturns for a given cycle the stakers taken into account …\nCheck if a denunciation has been executed given a …\nGet execution events optionally filtered by:\nGet a copy of a single datastore entry with its final and …\nGet the final and active values of balance.\nGet the execution status of a batch of operations.\nGet execution statistics\nPath to the hard drive cache storage\nMaximum number of entries we want to keep in the HD cache\nCalls <code>U::from(self)</code>.\nwhether the cycle is final\nexecution start state\nlast start period, used to attach to the correct execution …\nCost per byte in ledger\nLedger entry base cost\nLedger entry datastore base cost\nMaximum number of entries we want to keep in the LRU cache\nmapping grpc\nmaximum available gas for asynchronous messages execution\nMax bytecode size\nMax size of a datastore key\nMax datastore value size\nmax size of event data, in bytes\nMax execution traces slot to keep in trace history cache\nmaximum number of SC output events kept in cache\nMax function length in call sc\nMaximum gas to spend in the execution.\nMaximum gas to spend in the execution.\nmaximum gas per block\nMax miss ratio for auto roll sell\nMax parameter length in call sc\nMax gas for read only executions\nDatastore (key value store) for <code>ExecuteSC</code> Operation\noperation validity period\nOutput of a single execution\nList of addresses owned by the current call, and on which …\nParameter to pass to the target function\nperiods per cycle\nproduction stats\nAtomically query the execution state with multiple requests\nread-only execution request queue length\nList of requests\nList of responses\nNumber of roll to remove per denunciation\nprice of a roll inside the network\nAddress of the creator of the parent in the same thread\nThis module provides the structures used to provide …\nslot\nBroadcast channel for new slot execution outputs\nAmount of entries removed when <code>hd_cache_size</code> is reached\ninfos for each PoS-participating address among the ones …\nstate changes caused by the execution step\nduration of the statistics time window\nStop the execution thread Note that we do not take self by …\nStorage referencing the block and its contents\nStorage cost constants\nperiod duration\nTarget of the request\nTarget address\nTarget function\nnumber of threads\nThis file exports useful types used to interact with the …\ntypes for execution-trace / execution-info\nUpdates blockclique status by signaling newly finalized …\nexecution context in which the error happened\n<code>massa-sc-runtime</code> virtual machine error\nAddress for which to query the datastore\nAddress for which to query the datastore\nAddress for which to query the datastore\nAddress for which to query the datastore\ncycle to query\nKey of the entry\nKey of the entry\nFilter only entries whose key starts with a prefix\nFilter only entries whose key starts with a prefix\noptionally restrict the query to a set of addresses. If …\nParameter to pass to the target function\nTarget address\nTarget function\nchannels used by the execution worker\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nBroadcast channel for new slot execution outputs\ninterface that communicates with the execution worker …\nExecution manager used to stop the execution thread\nReturns a boxed clone of self. Useful to allow cloning …\nExecute read-only SC function call without causing …\nGets information about a batch of addresses\nReturns for a given cycle the stakers taken into account …\nCheck if a denunciation has been executed given a …\nGet execution events optionally filtered by:\nGet a copy of a single datastore entry with its final and …\nGet the final and active values of balance.\nGet the execution status of a batch of operations.\nGet execution statistics\nAtomically query the execution state with multiple requests\nStop the execution thread Note that we do not take self by …\nUpdates blockclique status by signaling newly finalized …\nBlock gas error: {0}\nCache error: {0}\nChannel error\nErrors of the execution component.\nExecution query errors\nFactory error: {0}\nInclude denunciation error: {0}\nInclude operation error: {0}\nInvalid slot range\n<code>MassaHashError</code>: {0}\n<code>ModelsError</code>: {0}\nNot enough gas in the block: {0}\nNot found: {0}\n<code>RollBuy</code> error: {0}\n<code>RollSell</code> error: {0}\nRuntime error: {0}\nSlash roll or deferred credits  error: {0}\nGiven gas is above the threshold: {0}\n<code>Transaction</code> error: {0}\nVM Error in {context} context: {error}\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nexecution context in which the error happened\n<code>massa-sc-runtime</code> virtual machine error\nStore for events emitted by smart contracts\nClear the event store\nExtend the event store with another store\nSet the events of this store as final\nReturns the argument unchanged.\nGet events optionally filtered by:\nCalls <code>U::from(self)</code>.\nPrune the event store if its size is over the given limit\nPush a new smart contract event to the store\nTake the event store\nConvert a vector of <code>grpc_model::ScExecutionEventsFilter</code> to …\nConverts a <code>ExecutionQueryResponse</code> to a …\nConvert a <code>grpc_api::ScExecutionEventsRequest</code> to a …\nExecution module configuration\nStorage cost constants\nconstant cost for async messages\nGas used by a transaction, a roll buy or a roll sell)\nWhere to dump blocks\nblock creation reward\nwhether slot execution outputs broadcast is enabled\nslot execution outputs channel capacity\nslot execution traces channel capacity\nwhether slot execution traces broadcast is enabled\nchain id\nextra lag to add on the execution cursor to improve …\nDenunciation expire delta\nendorsement count\nReturns the argument unchanged.\nReturns the argument unchanged.\nGas costs\ngenesis timestamp\nPath to the hard drive cache storage\nMaximum number of entries we want to keep in the HD cache\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nlast start period, used to attach to the correct execution …\nCost per byte in ledger\nLedger entry base cost\nLedger entry datastore base cost\nMaximum number of entries we want to keep in the LRU cache\nmaximum available gas for asynchronous messages execution\nMax bytecode size\nMax size of a datastore key\nMax datastore value size\nmax size of event data, in bytes\nMax execution traces slot to keep in trace history cache\nmaximum number of SC output events kept in cache\nMax function length in call sc\nmaximum gas per block\nMax miss ratio for auto roll sell\nMax parameter length in call sc\nMax gas for read only executions\noperation validity period\nperiods per cycle\nread-only execution request queue length\nNumber of roll to remove per denunciation\nprice of a roll inside the network\nAmount of entries removed when <code>hd_cache_size</code> is reached\nduration of the statistics time window\nStorage cost constants\nperiod duration\nnumber of threads\ngets the balance (candidate) of an address, returns …\ngets the balance (final) of an address, returns …\ngets the bytecode (candidate) of an address, returns …\ngets the bytecode (final) of an address, returns …\ngets the datastore keys (candidate) of an address, returns …\ngets the datastore keys (final) of an address, returns …\ngets a datastore value (candidate) for an address, returns …\ngets a datastore value (final) for an address, returns …\ngets the deferred credits (candidate) of an address, …\ngets the deferred credits (final) of an address, returns …\nchecks if address exists (candidate) returns …\nchecks if address exists (final) returns …\ngets the roll count (candidate) of an address, returns …\ngets the roll count (final) of an address, returns …\nThe operation or denunciation was found as executed with …\nThe operation or denunciation was found as successfully …\namount value\nboolean value\nbytecode\nExecute the main function of a bytecode\nget all information for a given cycle, returns …\ncycle infos value\ndatastore value\ndeferred credits value\ngets the execution status (candidate) for an denunciation, …\ngets the execution status (final) for an denunciation, …\nget filtered events. Returns …\nEvents\nNo information about the operation or denunciation …\nstructure storing a block id + network versions (from a …\nExecuted slot output\nExecution info about an address\nMetadata needed to execute the block\nstructure describing the output of a single execution\nInformation about cycles\nExecution status of an operation or denunciation\nRequest to atomically execute a batch of execution state …\nExecution state query item\nResponse to a list of execution queries\nExecution state query response item\nStaker information for a given cycle\nStructure describing an element of the execution stack. …\nexecution status value\nFinalized slot output\nExecute a function call\nlist of keys\ngets the execution status (candidate) for an operation, …\ngets the execution status (final) for an operation, …\nstructure describing a read-only call\nstructure describing the output of a read only execution\nstructure describing different types of read-only …\nstructure describing different possible targets of a …\nroll counts value\nstructure describing the output of the execution of a slot\nactive roll count\nCalled address\nAnnounced network version (see Versioning doc)\nAuto sell roll execution (empty if execution-info feature …\nBlock id\noptional executed block info at that slot (None if miss)\nReturned value from the module call\nCall stack to simulate, older caller first\nCall stack to simulate, older caller first. Target should …\nCancel async message execution (empty if execution-info …\ncandidate balance of the address\nLast executed candidate slot\ncandidate datastore keys of the address\ncandidate number of rolls the address has\nCoins transferred to the target address during the call\nCoins transferred to the target address during the call\nCurrent network version (see Versioning doc)\ncycle number\ncycle information\nDeferred credits execution (empty if execution-info …\nevents emitted by the execution step\nFee\nfinal balance of the address\nLast executed final slot\nfinal datastore keys of the address\nfinal number of rolls the address has\nFinal state hash\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nfuture deferred credits\nGas cost for this execution, with needed adjustments\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nwhether the cycle is final\nexecution start state\nMaximum gas to spend in the execution.\nMaximum gas to spend in the execution.\nDatastore (key value store) for <code>ExecuteSC</code> Operation\nOutput of a single execution\nList of addresses owned by the current call, and on which …\nParameter to pass to the target function\nproduction stats\nList of requests\nList of responses\nAddress of the creator of the parent in the same thread\nslot\ninfos for each PoS-participating address among the ones …\nstate changes caused by the execution step\nStorage referencing the block and its contents\nTarget of the request\nTarget address\nTarget function\nAddress for which to query the datastore\nAddress for which to query the datastore\nAddress for which to query the datastore\nAddress for which to query the datastore\ncycle to query\nKey of the entry\nKey of the entry\nFilter only entries whose key starts with a prefix\nFilter only entries whose key starts with a prefix\noptionally restrict the query to a set of addresses. If …\nParameter to pass to the target function\nTarget address\nTarget function")