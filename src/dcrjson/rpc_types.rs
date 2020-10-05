//! RPC Types.
//! Decred JSON RPC notification commands. Also contains standard commands to interact with lower versions such
//! as bitcoind.

/// Notification from the chain server that a block has been connected.
pub(crate) const NOTIFICATION_METHOD_BLOCK_CONNECTED: &str = "blockconnected";
/// Notification from the chain server that a block has been disconnected.
pub(crate) const NOTIFICATION_METHOD_BLOCK_DISCONNECTED: &str = "blockdisconnected";
/// Notifies a client when new tickets have matured.
pub(crate) const NOTIFICATION_METHOD_NEW_TICKETS: &str = "newtickets";
/// Notification that a new block has been generated.
pub(crate) const NOTIFICATION_METHOD_WORK: &str = "work";

/// Issues a notify blocks command to RPC server.
pub(crate) const METHOD_NOTIFY_BLOCKS: &str = "notifyblocks";
/// Issues a notify on new tickets command to RPC server.
pub(crate) const METHOD_NOTIFY_NEW_TICKETS: &str = "notifynewtickets";
/// Registers the client to receive notifications when a new block template has been generated
pub(crate) const METHOD_NOTIFIY_NEW_WORK: &str = "notifywork";
/// Returns information about the current state of the block chain.
pub(crate) const METHOD_GET_BLOCKCHAIN_INFO: &str = "getblockchaininfo";
/// Returns the number of blocks in the longest block chain.
pub(crate) const METHOD_GET_BLOCK_COUNT: &str = "getblockcount";
/// Returns hash of the block in best block chain at the given height.
pub(crate) const METHOD_GET_BLOCK_HASH: &str = "getblockhash";
