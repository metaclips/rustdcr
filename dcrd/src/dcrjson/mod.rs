pub mod chain_command_result;
mod errors;
pub use errors::RpcServerError;
pub mod future_types;
pub(crate) mod rpc_types;

use crate::chaincfg::chainhash::Hash;
use log::warn;

/// Parse hex string to bytes
pub(crate) fn parse_hex_parameters(value: &serde_json::Value) -> Option<Vec<u8>> {
    if value.is_null() {
        return Some(Vec::new());
    }

    let s: String = match serde_json::from_value(value.clone()) {
        Ok(val) => val,

        Err(e) => {
            warn!("Error unmarshalling hex parameters, error: {}", e);
            return None;
        }
    };

    match ring::test::from_hex(s.as_str()) {
        Ok(v) => return Some(v),

        Err(e) => {
            warn!("Error converting unmarshalled string to hex, error: {}", e);
            return None;
        }
    };
}

pub(crate) fn marshal_to_hash(value: serde_json::Value) -> Option<Hash> {
    let hash_string: String = match serde_json::from_value(value) {
        Ok(e) => e,

        Err(e) => {
            warn!("Error unmarshalling hash string, error: {}", e);
            return None;
        }
    };

    let hash = match crate::chaincfg::chainhash::Hash::new_from_str(&hash_string) {
        Ok(e) => e,

        Err(e) => {
            warn!("Error converting hash string to chain hash, error: {}", e);
            return None;
        }
    };

    Some(hash)
}
