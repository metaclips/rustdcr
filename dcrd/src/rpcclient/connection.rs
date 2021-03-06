//! Client connection.
//! Consists of all websocket cofigurations.

use {
    super::RpcClientError,
    futures::{stream::SplitStream, StreamExt},
    httparse::Status,
    log::warn,
    tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpStream,
        sync::mpsc,
    },
    tokio_tungstenite::{
        stream::Stream,
        tungstenite::{handshake::client::Request, handshake::headers, Message},
        MaybeTlsStream, WebSocketStream,
    },
};

/// Describes the connection configuration parameters for the client.
#[derive(Debug)]
pub struct ConnConfig {
    /// Full websocket url which consists host and port.
    pub host: String,

    /// Username to authenticate to the RPC server.
    pub user: String,

    /// Password to authenticate to the rpc server.
    pub password: String,

    /// Usually specified as `ws`.
    pub endpoint: String,

    /// Strings for a PEM-encoded certificate chain used
    /// for the TLS connection.  It has no effect if the DisableTLS parameter
    /// is true.
    pub certificates: String,

    /// Full socks5 proxy url containing `scheme` usually `Socks5`, `host` and `port` if specified.
    pub proxy_host: Option<String>,

    /// Username to connect to proxy.
    pub proxy_username: String,

    /// Password to connect to proxy.
    pub proxy_password: String,

    /// Specifies whether transport layer security should be
    /// disabled.  It is recommended to always use TLS if the RPC server
    /// supports it as otherwise your username and password is sent across
    /// the wire in cleartext.
    pub disable_tls: bool,

    /// Specifies that a websocket client connection should not be started
    /// when creating the client with `rpcclient::client::new`. Instead, the
    /// client is created and returned unconnected. `Connect` method must be called
    /// to start the websocket.
    pub disable_connect_on_new: bool,

    /// Disable reconnection if websocket fails.
    pub disable_auto_reconnect: bool,

    /// Instructs the client to run using multiple independent
    /// connections issuing HTTP POST requests instead of using the default
    /// of websockets.  Websockets are generally preferred as some of the
    /// features of the client such as notifications only work with websockets,
    /// however, not all servers support the websocket extensions, so this
    /// flag can be set to true to use basic HTTP POST requests instead.
    pub http_post_mode: bool,
}

impl Default for ConnConfig {
    fn default() -> Self {
        ConnConfig {
            certificates: String::new(),
            disable_connect_on_new: false,
            disable_tls: false,
            http_post_mode: false,
            disable_auto_reconnect: false,
            endpoint: String::from("ws"),
            host: "127.0.0.1:19109".to_string(),
            password: String::new(),
            proxy_host: None,
            proxy_username: String::new(),
            proxy_password: String::new(),
            user: String::new(),
        }
    }
}

impl ConnConfig {
    /// Creates a websocket connection and returns a websocket write feeder and a websocket reader. An asynchronous
    /// thread is spawn to forward messages sent from the ws_write feeder.
    pub async fn ws_split_stream(
        &mut self,
    ) -> Result<
        (
            SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
            mpsc::Sender<Message>,
        ),
        RpcClientError,
    > {
        let ws = match self.dial_websocket().await {
            Ok(ws) => ws,

            Err(e) => return Err(e),
        };

        // Split websocket to a sink which sends websocket messages to server and a stream which receives websocket messages.
        let (ws_sender, ws_receiver) = ws.split();

        // A bounded channel that forwards messages to the websocket sender.
        let (ws_tx, ws_rx) = mpsc::channel(1);

        // websocket receiver ws_rx is consumed here and is closed if websocket is closed.
        tokio::spawn(ws_rx.map(Ok).forward(ws_sender));

        Ok((ws_receiver, ws_tx))
    }

    /// Invokes a websocket stream to rpcclient using optional TLS and socks proxy.
    async fn dial_websocket(
        &mut self,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, RpcClientError> {
        let mut buffered_header = Vec::<u8>::new();

        let stream = match self.proxy_host.clone() {
            Some(proxy) => {
                self.add_proxy_header(&mut buffered_header);
                self.connect_stream(proxy.as_str()).await
            }

            None => self.connect_stream(self.host.clone().as_str()).await,
        };

        match stream {
            Ok(mut stream) => {
                if self.proxy_host.is_some() {
                    if let Err(e) = self
                        .dial_connection(&mut buffered_header, &mut stream)
                        .await
                    {
                        return Err(e);
                    }
                }

                let scheme = if self.disable_tls { "ws" } else { "wss" };
                let host = format!("{}://{}/{}", scheme, self.host, self.endpoint);

                let login = format!("{}:{}", self.user, self.password);
                let enc = base64::encode(login.as_bytes());
                let form = format!("Basic {}", enc);

                let wrapped_request = Request::builder()
                    .uri(host)
                    .header("authorization", form)
                    .body(());

                match wrapped_request {
                    Ok(request) => {
                        match tokio_tungstenite::client_async(request, stream).await {
                            Ok(websokcet) => {
                                return Ok(websokcet.0);
                            }

                            Err(e) => {
                                warn!("Error creating websocket handshake, error: {}", e);
                                return Err(RpcClientError::RpcHandshake(e));
                            }
                        };
                    }

                    Err(e) => {
                        warn!("Error building RPC authenticating request, error: {}.", e);

                        return Err(RpcClientError::RpcAuthenticationRequest);
                    }
                }
            }

            Err(e) => return Err(e),
        }
    }

    /// Upgrades stream connection to a secured layer.
    /// Add to create stream from should be specified in addr parameter.
    async fn connect_stream(
        &mut self,
        addr: &str,
    ) -> Result<MaybeTlsStream<TcpStream>, RpcClientError> {
        let tcp_stream = match tokio::net::TcpStream::connect(addr).await {
            Ok(tcp_stream) => tcp_stream,

            Err(e) => {
                warn!("Error connecting to tcp stream, error: {}", e);
                return Err(RpcClientError::TcpStream(e));
            }
        };

        if self.disable_tls {
            return Ok(Stream::Plain(tcp_stream));
        }

        let mut tls_connector_builder = native_tls::TlsConnector::builder();

        match native_tls::Certificate::from_pem(self.certificates.as_bytes()) {
            Ok(certificate) => {
                // ToDo: check if host name is an ip before accepting invalid hostname.
                tls_connector_builder
                    .add_root_certificate(certificate)
                    .min_protocol_version(native_tls::Protocol::Tlsv12.into())
                    .danger_accept_invalid_certs(true);
            }

            Err(e) => {
                warn!("Error parsing tls certificate, error: {}", e);
                return Err(RpcClientError::WsTlsCertificate(e));
            }
        }

        let wrapped_tls_stream = match tls_connector_builder.build() {
            Ok(tls_connector) => {
                tokio_native_tls::TlsConnector::from(tls_connector)
                    .connect(addr, tcp_stream)
                    .await
            }

            Err(e) => {
                warn!("Error creating tls handshake, error: {}", e);
                return Err(RpcClientError::TlsHandshake(e));
            }
        };

        match wrapped_tls_stream {
            Ok(tls_stream) => return Ok(Stream::Tls(tls_stream)),

            Err(e) => {
                warn!("Error creating tls stream, error: {}", e);
                return Err(RpcClientError::TlsStream(e));
            }
        }
    }

    /// Initiates proxy connection if proxy credentials are specified. CONNECT header is sent
    /// to proxy server using socks5.
    fn add_proxy_header(&mut self, buffered_header: &mut Vec<u8>) {
        buffered_header.extend_from_slice(
            format!(
                "\
            CONNECT {host} HTTP/1.1\r\n\
            Host: {host}\r\n\
            Proxy-Connection: Keep-Alive\r\n",
                host = self.host,
            )
            .as_bytes(),
        );

        // Add Authorization to proxy server passing basic auth credentials to stream header.
        let login = format!("{}:{}", self.user, self.password);

        let mut header_string = String::from("Basic ");
        header_string.push_str(&base64::encode(login.as_str()));

        buffered_header.extend_from_slice(
            &format!("{}: {}\r\n", "proxy-authorization", header_string).as_bytes(),
        );

        // Add trailing empty line
        buffered_header.extend_from_slice(b"\r\n");
    }

    /// Dials stream connection, sending http header to stream if user is using a proxy server for websocket connection.
    async fn dial_connection(
        &self,
        buffered_header: &mut Vec<u8>,
        stream: &mut MaybeTlsStream<TcpStream>,
    ) -> Result<(), RpcClientError> {
        match stream.write_all(buffered_header).await {
            Ok(_) => {}

            Err(e) => {
                warn!(
                    "Error writing request header to proxied stream, error: {}",
                    e
                );
                return Err(RpcClientError::ProxyAuthentication(e));
            }
        };

        let mut read_buffered = Vec::<u8>::new();

        loop {
            match stream.read_to_end(&mut read_buffered).await {
                Ok(_) => {}

                Err(e) => {
                    warn!(
                        "Error reading proxied RPC server received bytes, error: {}.",
                        e
                    );
                    return Err(RpcClientError::ProxyAuthentication(e));
                }
            };

            let mut header_buffer = [httparse::EMPTY_HEADER; headers::MAX_HEADERS];
            let mut response = httparse::Response::new(&mut header_buffer);

            match response.parse(&read_buffered) {
                Ok(val) => match val {
                    Status::Partial => continue,

                    Status::Complete(_) => match response.code {
                        Some(200) => return Ok(()),

                        _ => {
                            warn!(
                                "HTTP status error from proxied websocket, error code: {:?}.",
                                response.code
                            );

                            return Err(RpcClientError::RpcProxyStatus(response.code));
                        }
                    },
                },

                Err(e) => return Err(RpcClientError::RpcProxyResponseParse(e)),
            };
        }
    }
}
