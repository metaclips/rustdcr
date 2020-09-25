use crate::rpcclient::constants;
use std::collections::HashMap;

/// NotificationHandlers defines callback function pointers to invoke with
/// notifications.  Since all of the functions are None by default, all
/// notifications are effectively ignored until their handlers are set to a
/// concrete callback.
///
/// All callback functions are run async and are safe from blocking client requests.
pub struct NotificationHandlers {
    /// on_client_connected callback function is invoked when the client connects or
    /// reconnects to the RPC server.
    pub on_client_connected: Option<fn()>,

    /// on_block_connected callback function is invoked when a block is connected to the
    /// longest `best` chain.
    pub on_block_connected: Option<fn(block_header: Vec<u8>, transactions: Vec<Vec<u8>>)>,

    /// on_block_disconnected callback function is invoked when a block is disconnected from
    /// the longest `best` chain.
    pub on_block_disconnected: Option<fn(block_header: [u8])>,

    /// on_work callback function is invoked when a new block template is generated.
    pub on_work: Option<fn(data: [u8], target: [u8], reason: String)>,

    /// on_relevant_tx_accepted callback function is invoked when an unmined transaction passes
    /// the client's transaction filter.
    pub on_relevant_tx_accepted: Option<fn(transaction: [u8])>,

    /// on_reorganization callback function is invoked when the blockchain begins reorganizing.
    pub on_reorganization: Option<
        fn(
            old_hash: &[u8; constants::HASH_SIZE],
            old_height: i32,
            new_hash: &[u8; constants::HASH_SIZE],
            new_height: i32,
        ),
    >,

    /// on_winning_tickets callback function is invoked when a block is connected and eligible tickets
    /// to be voted on for this chain are given.
    pub on_winning_tickets:
        Option<fn(block_hash: i64, tickets: Vec<&[u8; crate::rpcclient::constants::HASH_SIZE]>)>,

    /// on_spent_and_missed_tickets callback function is invoked when a block is connected to the
    /// longest `best` chain and tickets are spent or missed.
    pub on_spent_and_missed_tickets: Option<
        fn(
            hash: &[u8; constants::HASH_SIZE],
            height: i64,
            stake_diff: i64,
            tickets: HashMap<[u8; constants::HASH_SIZE], bool>,
        ),
    >,

    /// on_new_tickets callback function is invoked when a block is connected to the longest `best` chain
    /// and tickets have matured and become active.
    pub on_new_tickets:
        Option<fn(height: i64, stake_diff: i64, tickets: Vec<&[u8; constants::HASH_SIZE]>)>,

    /// on_stake_difficulty callback function is invoked when a block is connected to the longest `best` chain
    /// and a new difficulty is calculated.
    pub on_stake_difficulty:
        Option<fn(hash: &[u8; constants::HASH_SIZE], height: i64, stake_diff: i64)>,

    /// on_unknown_notification callback function is invoked when an unrecognized notification is received.
    /// This typically means the notification handling code for this package needs to be updated for a new
    /// notification type or the caller is using a custom notification this package does not know about.
    pub on_unknown_notification: Option<fn(method: String, params: [u8])>,
}

impl Default for NotificationHandlers {
    fn default() -> Self {
        NotificationHandlers {
            on_block_connected: None,
            on_block_disconnected: None,
            on_client_connected: None,
            on_new_tickets: None,
            on_relevant_tx_accepted: None,
            on_reorganization: None,
            on_spent_and_missed_tickets: None,
            on_stake_difficulty: None,
            on_unknown_notification: None,
            on_winning_tickets: None,
            on_work: None,
        }
    }
}

/// Used to track the current state of successfully registered notifications so the
/// state can be automatically re-established on reconnect.
pub(super) struct NotificationState {
    pub(super) notify_blocks: bool,
    pub(super) notify_work: bool,
    pub(super) notify_winning_tickets: bool,
    pub(super) notify_spent_and_missed_tickets: bool,
    pub(super) notify_new_tickets: bool,
    pub(super) notify_stake_difficulty: bool,
    pub(super) notify_new_tx: bool,
    pub(super) notify_new_tx_verbose: bool,
}

impl Default for NotificationState {
    fn default() -> Self {
        NotificationState {
            notify_blocks: false,
            notify_work: false,
            notify_winning_tickets: false,
            notify_spent_and_missed_tickets: false,
            notify_new_tickets: false,
            notify_stake_difficulty: false,
            notify_new_tx: false,
            notify_new_tx_verbose: false,
        }
    }
}
