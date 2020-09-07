use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Instant;

use tokio::time::Duration;
use websocket::{ClientBuilder, Message, WebSocketError};
use websocket::client::sync::Client;
use websocket::url::Url;
use websocket::websocket_base::stream::sync::{TcpStream, TlsStream};
use websocket::ws::dataframe::DataFrame;

use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::traits::message_parser::MessageParser;
use crate::logger::Logger;
use crate::user::oauth_token::OauthToken as UserOauthToken;
use crate::user::user_data::Data as UserData;
use crate::user::user_properties::UserLogin;


pub struct WebSocketSession<TParser, TLogger>
    where TParser: MessageParser<TLogger>,
        TLogger: Logger {
    irc_url:Url,
    message_parser: TParser,
    client_user: UserData,
    user_token: UserOauthToken,
    logger: TLogger,
    tick_rate_ms:u64,
    last_tick:Instant
}

unsafe impl<TParser, TLogger> Sync for WebSocketSession<TParser,TLogger> where TParser: MessageParser<TLogger>,
                                                                               TLogger: Logger {}

unsafe impl<TParser, TLogger> Send for WebSocketSession<TParser,TLogger> where TParser: MessageParser<TLogger>,
                                                                               TLogger: Logger {}


impl<TParser,TLogger> WebSocketSession<TParser, TLogger>
    where TParser: MessageParser<TLogger> ,
          TLogger: Logger {
    //  Constructor
    pub fn new(client_user: UserData, new_token: UserOauthToken, new_message_parser: TParser, new_logger: TLogger, url: Url) -> WebSocketSession<TParser,TLogger> {
        WebSocketSession { irc_url: url, client_user: client_user, user_token: new_token, message_parser: new_message_parser, logger: new_logger, tick_rate_ms: 1000, last_tick: Instant::now() }
    }


    // Init
    //
    pub async fn initialize<TOnStartFunction>(self_arc:Arc<Mutex<Self>>, on_start_function:TOnStartFunction)
        where TOnStartFunction: FnOnce(&mut Self, &mut Client<TlsStream<TcpStream>>) {


        //let listen_future = Self::begin_continuous_listen(self_arc.clone(), &mut irc_listener);

        let self_arc1 = self_arc.clone();

        let mut irc_listener;
        let url;
        {
            let local_arc = self_arc;

            let local_mutex = local_arc.deref().lock().unwrap();
            let mut self_ref = local_mutex;

            url = self_ref.irc_url.clone();
            irc_listener = ClientBuilder::from_url(&url).connect_secure(None).unwrap();

            on_start_function(&mut self_ref, &mut irc_listener);
        }

        // listen thread
        tokio::task::spawn_blocking(move || {

            println!("start listen");

            let self_arc = self_arc1.clone();

            let mut irc_listener = irc_listener;

            sleep(Duration::from_millis(2));

            loop {
                println!("lsiten outer");


                WebSocketSession::listen(self_arc.clone(), &mut irc_listener);


                println!("Listen thread sleeping for 2ms");
                sleep(Duration::from_millis(2));
            }
        });
    }
    //
    fn listen(self_arc:Arc<Mutex<Self>>, irc_listener:&mut Client<TlsStream<TcpStream>>) {

       println!("listen waiting for message...");

        // get received data as a &str
        let received_result = irc_listener.recv_message();
        {
            if received_result.is_err() {
                let error = received_result.unwrap_err();
                match error {
                    WebSocketError::NoDataAvailable => {}
                    _ => {
                        println!("IRC client received the error: {}", error.to_string().as_str());
                    }
                }
                return;
            }
        }
        println!("listen received message.");
        let received_msg = received_result.unwrap();
        let received_data = received_msg.take_payload();
        let received_string = String::from_utf8(received_data).unwrap();

        {
            match self_arc.try_lock() {
                Ok(local_mutex) => { local_mutex.register_received_data(irc_listener, received_string.as_str()); },
                Err(e) => { panic!("Could not register message '{0}' ERROR: {1}", received_string, e); },
            }
        }
        println!("RCVD: {}", received_string);
    }

    pub fn send_string(&self, irc_dispatcher:&mut Client<TlsStream<TcpStream>>, data_to_send: String) {
        irc_dispatcher.send_message(&Message::text(data_to_send.clone())).unwrap();

        if !data_to_send.contains("PASS") && !data_to_send.contains("auth_token:") {
            println!("SENT: {}", data_to_send);
        }
    }

    fn register_received_data(&self, irc_dispatcher:&mut Client<TlsStream<TcpStream>>, received_data: &str) {
        const NEW_LINE: &str = "\r\n";

        for line in received_data.split(NEW_LINE) {
            if line.is_empty() { continue; }

            self.logger.write_line(format!("RCVD: {0}", line));

            self.process_response(irc_dispatcher, line.to_string());
        }
    }

    pub fn process_response (&self, irc_dispatcher:&mut Client<TlsStream<TcpStream>>, response:String) {
        let mut context = ResponseContext::new(self.client_user.clone(), response.to_string());
        if !self.message_parser.process_response(&mut context, &self.logger) {
            self.logger.write_line(format!("IRC PARSER FAILED TO READ LINE: {0}", response));
        } else {
            for response_to_send in context.get_responses_to_reply_with() {
                self.send_string(irc_dispatcher, response_to_send);
            }
        }
    }

    pub fn join_chat_channel(&mut self, irc_dispatcher:&mut Client<TlsStream<TcpStream>>, user_login: UserLogin) {
        const JOIN_PREFIX: &str = "JOIN #";

        self.send_string(irc_dispatcher, format!("{0}{1}", JOIN_PREFIX, user_login.get_value()));
    }
}