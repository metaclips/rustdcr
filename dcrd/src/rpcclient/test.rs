#![allow(missing_docs)]
// ToDo: Add test for RPC proxie and RPC
// Add test for Notification handling
// Add test for notification reregistration
// Add test for commands both on HTTP and normal.

mod conntest {
    use async_trait::async_trait;
    use futures_util::{
        stream::{SplitSink, SplitStream, StreamExt},
        SinkExt,
    };
    use tokio::sync::mpsc;
    use tokio_tungstenite::{
        accept_hdr_async, connect_async,
        tungstenite::{
            handshake::{client::Request, server::Response},
            Message,
        },
    };

    use crate::{
        dcrjson::{commands, result_types::JsonResponse},
        rpcclient::{self, connection::Websocket, error::RpcClientError, infrastructure::Command},
    };
    use tokio_tungstenite::tungstenite::error;

    #[tokio::test]
    async fn test_conn() {
        println!("starting test");
        let (sender, mut recvr) = tokio::sync::mpsc::channel(1);
        let url = "127.0.0.1:3000";

        tokio::spawn(async {
            _start_server(url, sender).await;
            println!("server stopped");
        });

        use crate::rpcclient::{client, notify::NotificationHandlers};

        recvr.recv().await.unwrap();
        println!("recvd");

        let mut test_client = client::new(
            WebsocketConnTest {
                url: url.to_string(),
            },
            NotificationHandlers::default(),
        )
        .await
        .unwrap();

        test_client.disconnect().await;

        // TODO: Try sending request here.
        match test_client.get_block_count().await.err().unwrap() {
            RpcClientError::RpcDisconnected => println!("client disconnected"),
            e => panic!("rpcclient client not disconnected: {}", e),
        }

        assert!(
            test_client.is_disconnected().await,
            "websocket wasnt disconnected"
        );

        match test_client.connect().await {
            Ok(_) => println!("websocket reconnected"),
            Err(e) => panic!("websocket errored reconnecting: {}", e),
        };

        test_client.get_block_count().await.unwrap().await.unwrap();

        test_client.shutdown().await;
    }

    #[tokio::test]
    async fn test_invalid_notification() {
        println!("starting test");
        let (sender, mut recvr) = tokio::sync::mpsc::channel(1);
        let url = "127.0.0.1:3001";

        tokio::spawn(async {
            _start_server(url, sender).await;
            println!("server stopped");
        });

        use crate::rpcclient::{client, notify::NotificationHandlers};

        recvr.recv().await.unwrap();

        let mut test_client = client::new(
            WebsocketConnTest {
                url: url.to_string(),
            },
            NotificationHandlers::default(),
        )
        .await
        .unwrap();

        let result = test_client.notify_new_transactions(true).await;
        assert!(result.is_err());

        assert_eq!(
            format!("{}", result.err().unwrap()),
            format!(
                "{}",
                RpcClientError::UnregisteredNotification(
                    commands::METHOD_NOTIFY_NEW_TX.to_string()
                )
            )
        );

        test_client.shutdown().await;
    }

    /// Implements JSON RPC request structure to server.
    #[derive(serde::Deserialize)]
    pub struct TestRequest<'a> {
        pub jsonrpc: &'a str,
        pub method: &'a str,
        pub id: u64,
        pub params: Vec<serde_json::Value>,
    }

    #[derive(Clone)]
    struct WebsocketConnTest {
        pub url: String,
    }

    fn _mock_get_block_count(id: u64) -> Message {
        let res = JsonResponse {
            id: serde_json::json!(id),
            method: serde_json::json!(commands::METHOD_GET_BLOCK_COUNT),
            result: serde_json::json!(100),
            params: Vec::new(),
            error: serde_json::Value::Null,
            ..Default::default()
        };

        let marshalled = serde_json::to_string(&res).unwrap();
        Message::Text(marshalled)
    }

    async fn _start_server(url: &str, ready: tokio::sync::mpsc::Sender<()>) {
        let server = tokio::net::TcpListener::bind(url)
            .await
            .expect("unable to bind");

        println!("Server listening");

        ready.send(()).await.expect("error sending ready signal");

        println!("looking for connections");

        loop {
            if let Ok(stream) = server.accept().await {
                let callback = |req: &Request, response: Response| {
                    println!("Received a new ws handshake");
                    println!("The request's path is: {}", req.uri().path());
                    println!("The request's headers are:");
                    for (ref header, _value) in req.headers() {
                        println!("* {}", header);
                    }

                    // Let's add an additional header to our response to the client.
                    // let headers = response.headers_mut();
                    // headers.append("MyCustomHeader", ":)".parse().unwrap());
                    // headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                    Ok(response)
                };

                let websocket = accept_hdr_async(stream.0, callback).await.unwrap();

                println!("found a conn on ip: {}", stream.1);
                let (mut write, mut read) = websocket.split();

                while let Some(msg) = read.next().await {
                    let msg = match msg {
                        Ok(msg) => msg,

                        Err(e) => match e {
                            error::Error::ConnectionClosed => break,
                            _ => panic!("connection closed abruptly: {}", e),
                        },
                    };

                    if msg.is_binary() || msg.is_text() {
                        let msg_to_str = &msg.to_string();
                        let res: TestRequest = serde_json::from_str(msg_to_str).unwrap();

                        match res.method {
                            commands::METHOD_GET_BLOCK_COUNT => {
                                write.send(_mock_get_block_count(res.id)).await.unwrap()
                            }
                            _ => unreachable!(),
                        };
                    } else if msg.is_close() {
                        println!("close message received");
                        break;
                    }
                }
            }
        }
    }

    #[async_trait]
    impl rpcclient::connection::RPCConn for WebsocketConnTest {
        async fn ws_split_stream(
            &mut self,
        ) -> Result<(SplitStream<Websocket>, SplitSink<Websocket, Message>), RpcClientError>
        {
            let (ws_stream, _) = connect_async(format!("ws://{}", self.url))
                .await
                .expect("Failed to connect");
            println!("WebSocket handshake has been successfully completed");

            let (ws_send, ws_rcv) = ws_stream.split();

            Ok((ws_rcv, ws_send))
        }

        fn disable_connect_on_new(&self) -> bool {
            false
        }

        fn is_http_mode(&self) -> bool {
            false
        }

        fn disable_auto_reconnect(&self) -> bool {
            false
        }

        async fn handle_post_methods(
            &self,
            _http_user_command: mpsc::Receiver<Command>,
        ) -> Result<(), RpcClientError> {
            todo!()
        }
    }
}
