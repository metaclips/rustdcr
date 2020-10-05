use rustdcr::{
    chaincfg::chainhash::Hash,
    dcrutil::app_data,
    rpcclient::{client, connection, notify},
};

use std::{fs, path::PathBuf};

#[tokio::main]
async fn main() {
    // Get dcrd app directory, if none is found use current path.
    let mut app_dir = match app_data::get_app_data_dir("dcrd".into(), false) {
        Some(dir) => dir,

        None => PathBuf::new().join("."),
    };

    app_dir.push("rpc.cert");

    let certs = fs::read_to_string(app_dir).unwrap();

    let config = connection::ConnConfig {
        host: "127.0.0.1:19109".to_string(),
        password: "rpcpassword".to_string(),
        user: "rpcuser".to_string(),
        certificates: certs,
        ..Default::default()
    };

    let notif_handler = notify::NotificationHandlers {
        on_client_connected: Some(|| {
            println!("client connected");
        }),

        on_block_connected: Some(|block_header: Vec<u8>, transactions: Vec<Vec<u8>>| {
            println!(
                "\n\n\n\t\t\t\tBlock Connected Notif\nBlock header: {:?} \n\nTransactions: {:?}\n\n\n",
                block_header, transactions,
            )
        }),

        on_block_disconnected: Some(|block_header: Vec<u8>| {
            println!(
                "\n\n\t\t\t\tBlock Disconnected Notif\n\nBlock header: {:?}\n\n\n",
                block_header,
            )
        }),

        on_work: Some(|data: Vec<u8>, target: Vec<u8>, reason: String| {
            println!(
                "\n\n\t\t\t\tOn Work Notif\n\nData: {:?}\n\n\nTransactions: {:?}\n\n\n\nReason: {:?}\n\n\n",
                data,target, reason,
            )
        }),

        on_new_tickets: Some(
            |hash: Hash, height: i64, stake_diff: i64, tickets: Vec<Hash>| {
                println!(
                    "\n\n\t\t\t\tOn Tickets Notif\n\n\nHash: {:?}\n\n\nHeight: {:?}\n\n\nStake Diff: {:?}\n\n\nTickets: {:?}\n\n\n\n",
                    hash.string().unwrap(),height, stake_diff,tickets,
                )
            },
        ),

        ..Default::default()
    };

    let mut client = client::new(config, notif_handler).await.unwrap();

    client
        .notify_work()
        .await
        .expect("Unable to send work notification command to server")
        .await
        .expect("Server replied with an error on notify work");

    client
        .notify_new_tickets()
        .await
        .expect("Unable to send new ticket notification command to server")
        .await
        .expect("Server replied with an error on notify work");

    client
        .notify_blocks()
        .await
        .expect("Unable to send block notification command to server")
        .await
        .expect("Server replied with an error on notify blocks");

    // Ensure command is sent to server.
    let blk_info = client
        .get_blockchain_info()
        .await
        .expect("Could not send get blockchain info request to server");

    let blk_count = client
        .get_block_count()
        .await
        .expect("Could not send get block count request to server");

    // Blockchain info is sent to a different async thread.
    tokio::spawn(async move {
        // Collect result from server and print result.
        let blk_info_result = blk_info
            .await
            .expect("Error getting blockchain info result");

        println!(
            "\n\n\n\nBest Block Hash {} \n\nBlocks {}
            \n\nChain {} \n\nChain Work {}
            \n\nDeployments {:?} \n\nDifficulty {}
            \n\nDifficulty Ratio {} \n\nHeaders {}
            \n\nInitial Block Download {} \n\nMax Block Size {}
            \n\nSync Height {} \n\nVerification Progress {}",
            blk_info_result.best_block_hash,
            blk_info_result.blocks,
            blk_info_result.chain,
            blk_info_result.chain_work,
            blk_info_result.deployments,
            blk_info_result.difficulty,
            blk_info_result.difficulty_ratio,
            blk_info_result.headers,
            blk_info_result.initial_block_download,
            blk_info_result.max_block_size,
            blk_info_result.sync_height,
            blk_info_result.verification_progress
        );

        let blk_count_result = blk_count.await.unwrap();
        println!("\n\n\nBlock Count: {}", blk_count_result);
    });

    client.wait_for_shutdown();
}
