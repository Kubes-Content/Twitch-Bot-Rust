use crate::IRC::MessageParser::IrcMessageParser as MessageParser;
use crate::User::UserProperties::UserLogin;
use crate::Logger::Logger;
use crate::User::OAuthToken::OauthToken as UserOauthToken;
use crate::OAuth::HasOauthSignature::HasOauthSignature;
use websocket::client::sync::Client;

use std::str::FromStr;
use websocket::url::Url;
use websocket::{Message, WebSocketError, ClientBuilder};
use websocket::ws::dataframe::DataFrame;
use tokio::time::Duration;
use websocket::stream::sync::TcpStream;
use futures::Future;


const IRC_TOKEN_PREFIX:&str = "PASS oauth:";
const IRC_USERNAME_PREFIX:&str = "NICK ";
const RECEIVE_BUFFER_SIZE:usize = 1024;
pub const TWITCH_IRC_URL:&str = "ws://irc-ws.chat.twitch.tv:80";

pub struct IrcChatSession<'life> {
    // Async Batcher (for i/o requests) = new AsyncBatcher();
    client:Client<TcpStream>,
    message_parser:& 'life mut dyn MessageParser,
    username:UserLogin, // the channel's username? the user's? Both?
    user_token:UserOauthToken,
    logger:&'life dyn Logger,
    receive_buffer:Vec<u8> // 1024 bytes
}
//
impl<'life> IrcChatSession<'life> {
    //  Constructor
    pub fn new(new_username:UserLogin, new_token:UserOauthToken, new_message_parser:&'life mut dyn MessageParser, new_logger:&'life dyn Logger, url:Url) -> IrcChatSession<'life> {
        IrcChatSession { client: ClientBuilder::from_url(&url).connect_insecure().unwrap(), username: new_username, user_token: new_token, message_parser: new_message_parser, logger: new_logger, receive_buffer: vec![0; RECEIVE_BUFFER_SIZE] }
    }

    // Init
    //
    pub async fn initialize(& mut self, channels_to_join:Vec<UserLogin>) {
        self.init_socket().await;
        self.authenticate_user().await;
        for channel in channels_to_join {
            self.join_channel(channel);
        }
        self.begin_continuous_listen().await;
    }
    //
    async fn init_socket(&mut self) {
        let url:Url = websocket::url::Url::from_str(TWITCH_IRC_URL).unwrap();

        self.client = ClientBuilder::from_url(&url).connect_insecure().unwrap();

        self.client.stream_ref();

        self.client.send_message(&Message::text("derp"));
    }
    //
    async fn authenticate_user(&mut self){
        self.send_string(&format!("{0}{1}", IRC_TOKEN_PREFIX, self.user_token.get_oauth_signature().to_string()));
        self.send_string(&format!("{0}{1}", IRC_USERNAME_PREFIX, self.username.get_value()));
    }
    //
    fn listen(&mut self) {
        // get received data as a &str
        let received_result = self.client.recv_message(); {
            if received_result.is_err() {
                let error = received_result.unwrap_err();
                match error {
                    WebSocketError::NoDataAvailable => { }
                    _ => {
                        println!("IRC client received the error: {}", error.to_string().as_str());
                    }
                }
                return
            }
        }
        let received_msg = received_result.unwrap();
        let received_data = received_msg.take_payload();
        let received_string = String::from_utf8(received_data).unwrap();

        self.register_received_data(received_string.as_str());
        //println!("RCVD: {}", received_string);
    }
    //
    async fn begin_continuous_listen(& mut self) {
        while true {

            // listen
            self.listen();

            // wait 10ms
            tokio::time::delay_for(Duration::from_millis(10)).await;
        }
    }
    //

    fn send_string(&mut self, data_to_send:&str){
        self.client.send_message(&Message::text(data_to_send));
        println!("SENT: {}", data_to_send);
    }

    fn register_received_data(&mut self, received_data:&str){
        const NEW_LINE:&str = "\r\n";

        for line in received_data.split(NEW_LINE) {

            if line.is_empty() { continue; }

            self.logger.write_line(format!("RCVD: {0}", line).as_str());
            let mut responses_to_send:Vec<&str> = Vec::new();

            if !self.message_parser.process_response(self.username.clone(), line, &mut responses_to_send, self.logger) {
                self.logger.write_line(format!("IRC PARSER FAILED TO READ LINE: {0}", line).as_str());
            } else {
                for response_to_send in responses_to_send{
                    self.send_string(response_to_send);
                }
            }
        }
    }

    pub fn join_channel(&mut self, user_login:UserLogin){
        const JOIN_PREFIX:&str = "JOIN #";

        self.send_string(&format!("{0}{1}", JOIN_PREFIX, user_login.get_value()));
    }
}