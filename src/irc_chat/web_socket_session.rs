use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Instant;

use tokio::time::{Duration, delay_for};
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
    pub async fn initialize<TOnStartFunction>(self_arc:Arc<tokio::sync::Mutex<Self>>, on_start_function:TOnStartFunction)
        where TOnStartFunction: FnOnce(&mut Self, &mut Client<TlsStream<TcpStream>>) {


        //let listen_future = Self::begin_continuous_listen(self_arc.clone(), &mut irc_listener);

        let self_arc1 = self_arc.clone();

        let mut irc_listener;
        let url;
        {
            let local_arc = self_arc;

            let local_mutex = local_arc.deref().lock().await;
            let mut self_ref = local_mutex;

            url = self_ref.irc_url.clone();
            irc_listener = ClientBuilder::from_url(&url).connect_secure(None).unwrap();

            on_start_function(&mut self_ref, &mut irc_listener);
        }

        // listen thread
        tokio::task::spawn(async move {

            println!("start listen");

            let self_arc = self_arc1.clone();

            let irc_listener = Arc::new(tokio::sync::Mutex::new(irc_listener));

            sleep(Duration::from_millis(2));

            loop {
                println!("listen outer");



                WebSocketSession::listen(self_arc.clone(), irc_listener.clone()).await;
            }
        });
    }
    //
    async fn listen(self_arc:Arc<tokio::sync::Mutex<Self>>, irc_listener:Arc<tokio::sync::Mutex<Client<TlsStream<TcpStream>>>>) {

        println!("listen inner");

        let received_result_arc = Arc::new(Mutex::new(None));

        let received_result_arc_local = received_result_arc.clone();

        let self_arc_local = self_arc.clone();

        let irc_local = irc_listener.clone();

        // message retrieval requires blocking
        tokio::task::spawn_blocking( move || {

            let received_result_arc = received_result_arc.clone();
            let irc_listener = irc_listener.clone();
            {
                match self_arc.try_lock() {
                    Ok(_local_mutex) => {
                        println!("listen waiting for message...");

                        //let mut irc_listener = ClientBuilder::from_url(&local_mutex.irc_url).connect_secure(None).unwrap();




                        match received_result_arc.try_lock() {
                            Ok(mut received_result) => {
                                // get received data as a &str

                                match irc_listener.try_lock() {
                                    Ok(mut irc_mutex) => {
                                        println!("attempt retrieval");
                                        *received_result = Some(irc_mutex.recv_message());
                                        println!("post retrieval");

                                    }
                                    Err(e) => {
                                        panic!("Could not lock irc! ERROR: {}", e);
                                    }
                                }



                                match received_result.as_ref().clone().unwrap() {
                                    Ok(msg) => { println!("received message!") },
                                    Err(e) => {
                                        match e {
                                            WebSocketError::NoDataAvailable => {},
                                            _ => {
                                                println!("IRC client received the error: {}", e);
                                            }
                                        }
                                        return;
                                    },
                                }
                            },
                            Err(e) => { panic!("Could not lock received_result {}", e); },
                        }

                    },
                    Err(e) => {
                        panic!("Could not lock self ERROR: {}", e);
                    },
                }
            }
        });

        // wait for retrieval
        delay_for(Duration::from_millis(1)).await;


        let received_result_arc = received_result_arc_local;
        let self_arc = self_arc_local;

        let mut is_blocking = true;
        while is_blocking {

            delay_for(Duration::from_millis(1)).await;

            match self_arc.try_lock() {
                Err(blocking) => {
                    //println!("Waiting for message to be received...");
                }
                _ => {
                    is_blocking = false;
                }
            }
        }

        let mut received_string = String::new();

        match received_result_arc.clone().try_lock() {
            Ok(received_result) => {
                println!("listen received message.");

                let received_msg_result = received_result.as_ref().clone().unwrap().as_ref();

                if received_msg_result.is_err()
                    && match received_msg_result.unwrap_err() {
                    WebSocketError::NoDataAvailable => {
                        return;
                    }
                    _ => { false }
                }
                {
                    return;
                }

                let received_msg = received_msg_result.unwrap().clone();
                let received_data = received_msg.take_payload();
                received_string = String::from_utf8(received_data).unwrap();

            },
            Err(e) => {
                panic!("Could not lock received result. ERRORL: {}", e)
            },
        }

        match self_arc.try_lock() {
            Ok(local_mutex) => {
                let f = local_mutex.register_received_data(irc_local, received_string.as_str()).await;
            }
            Err(e) => { panic!("Could not register message '{0}' ERROR: {1}", received_string, e); },
        }

        println!("RCVD: {}", received_string);

    }

    pub fn send_string(&self, irc_dispatcher:&mut Client<TlsStream<TcpStream>>, data_to_send: String) {
        match irc_dispatcher.send_message(&Message::text(data_to_send.clone())) {
            Ok(_) => {},
            Err(e) => { println!("ERROR: {}", e) },
        };

        if !data_to_send.contains("PASS") && !data_to_send.contains("auth_token:") {
            println!("SENT: {}", data_to_send);
        }
    }

    async fn register_received_data(&self, irc_dispatcher:Arc<tokio::sync::Mutex<Client<TlsStream<TcpStream>>>>, received_data: &str) {
        const NEW_LINE: &str = "\r\n";

        for line in received_data.split(NEW_LINE) {
            if line.is_empty() { continue; }

            self.logger.write_line(format!("RCVD: {0}", line));

            self.process_response(irc_dispatcher.clone(), line.to_string()).await;
        }



    }

    pub async fn process_response (&self, irc_dispatcher:Arc<tokio::sync::Mutex<Client<TlsStream<TcpStream>>>>, response:String) {
        let context_mutex = Arc::new(tokio::sync::Mutex::new(ResponseContext::new(self.client_user.clone(), response.to_string())));

        let read_successful = {
            self.message_parser.process_response(context_mutex.clone(), &self.logger).await
        };


        if !read_successful {
            self.logger.write_line(format!("IRC PARSER FAILED TO READ LINE: {0}", response));
        } else {
            match irc_dispatcher.try_lock() {
                Ok(mut irc_mutex) => {
                    match context_mutex.try_lock() {
                        Ok(context) => {
                            for response_to_send in context.get_responses_to_reply_with() {
                                self.send_string(&mut irc_mutex, response_to_send);
                            }
                        }
                        Err(e) => { panic!("ERROR! : {}", e) }
                    }
                }
                Err(e) => { panic!("Could not lock IRC, ERROR {}", e) }
            }
        }
    }

    pub fn join_chat_channel(&mut self, irc_dispatcher:&mut Client<TlsStream<TcpStream>>, user_login: UserLogin) {
        const JOIN_PREFIX: &str = "JOIN #";

        self.send_string(irc_dispatcher, format!("{0}{1}", JOIN_PREFIX, user_login.get_value()));
    }
}