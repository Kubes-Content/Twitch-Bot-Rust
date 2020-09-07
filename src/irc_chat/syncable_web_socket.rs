use websocket::{ClientBuilder, Message, OwnedMessage, WebSocketResult};
use websocket::client::sync::Client;
use websocket::url::Url;
use websocket::websocket_base::stream::sync::TcpStream;


pub struct SyncableClient {
    inner_client:Client<TcpStream>
}

unsafe impl Send for SyncableClient {}

unsafe impl Sync for SyncableClient {}

impl SyncableClient {
    pub fn new(url:Url) -> SyncableClient {
        match ClientBuilder::from_url(&url).connect_insecure() {
            Ok(client) => {
                SyncableClient { inner_client: client }
            },
            Err(error) => { panic!("ERROR: could not create client to url {0}.\n{1}", url.as_str(), error) },
        }
    }

    pub fn send_message(&mut self, message:&Message) -> WebSocketResult<()> {
        self.inner_client.send_message(message)
    }

    pub fn recv_message(&mut self) -> WebSocketResult<OwnedMessage> {
        self.inner_client.recv_message()
    }

    pub fn set_nonblocking(&self, val:bool) {
        self.inner_client.set_nonblocking(val).unwrap();
    }
}