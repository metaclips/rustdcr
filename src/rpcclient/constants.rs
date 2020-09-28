#[forbid(missing_docs)]
pub(super) const HASH_SIZE: usize = 32;
pub(super) const CONNECTION_RETRY_INTERVAL_SECS: std::time::Duration =
    std::time::Duration::from_secs(10);

/// SEND_BUFFER_SIZE is the number of elements the websocket send channel
/// can queue before blocking.
pub(super) const SEND_BUFFER_SIZE: usize = 50;

/// The required timeframe to send pings to websocket.
pub(super) const KEEP_ALIVE: u64 = 10;
