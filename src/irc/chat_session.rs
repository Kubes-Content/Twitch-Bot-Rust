use crate::irc::traits::message_parser::IrcMessageParser as MessageParser;
use crate::user::user_properties::UserLogin;
use crate::logger::{Logger, DefaultLogger};
use crate::user::oauth_token::OauthToken as UserOauthToken;
use crate::oauth::has_oauth_signature::HasOauthSignature;
use websocket::client::sync::{Client};

use websocket::url::Url;
use websocket::{Message, WebSocketError, ClientBuilder};
use websocket::ws::dataframe::DataFrame;
use tokio::time::{Duration, delay_for};
use websocket::stream::sync::TcpStream;
use crate::irc::response_context::ResponseContext;
use crate::user::user_data::Data as UserData;
use std::time::Instant;
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};
use futures::executor::block_on;
use std::ops::Deref;
use crate::irc::channel_chatter_data::ChatterData;
use futures::TryFutureExt;
use tokio::runtime::Runtime;
use crate::irc::default_irc_message_parser::DefaultMessageParser;
use crate::debug::fail_safely;
use websocket::futures::Async;
use tokio::io::Error;
use crate::irc::syncable_web_socket::SyncableClient;
use std::thread::sleep;
use tokio::task;


const IRC_TOKEN_PREFIX: &str = "PASS oauth:";
const IRC_USERNAME_PREFIX: &str = "NICK ";
//const RECEIVE_BUFFER_SIZE:usize = 1024;
pub const TWITCH_IRC_URL: &str = "ws://IRC-ws.chat.twitch.tv:80";

pub struct IrcChatSession<TParser:Send, TLogger:Send>
    where TParser: MessageParser<TLogger> + Clone + Sync + Send,
        TLogger: Logger + Clone + Sync + Send {
    irc_url:Url,
    message_parser: TParser,
    client_user: UserData,
    user_token: UserOauthToken,
    logger: TLogger,
    tick_rate_ms:u64,
    last_tick:Instant
}

unsafe impl<TParser, TLogger> Sync for IrcChatSession<TParser,TLogger> where TParser: MessageParser<TLogger> + Clone + Sync + Send,
                                                                             TLogger: Logger + Clone + Sync + Send {}

unsafe impl<TParser, TLogger> Send for IrcChatSession<TParser,TLogger> where TParser: MessageParser<TLogger> + Clone + Send + Sync,
                                                                             TLogger: Logger + Clone + Send + Sync {}


impl<TParser:Send,TLogger:Send> IrcChatSession<TParser, TLogger>
    where TParser: MessageParser<TLogger> + Clone + Send + Sync + 'static,
          TLogger: Logger + Clone + Send + Sync + 'static {
    //  Constructor
    pub fn new(client_user: UserData, new_token: UserOauthToken, new_message_parser: TParser, new_logger: TLogger, url: Url) -> IrcChatSession<TParser,TLogger> {
        IrcChatSession { irc_url: url, client_user: client_user, user_token: new_token, message_parser: new_message_parser, logger: new_logger, tick_rate_ms: 1000, last_tick: Instant::now() }
    }


    pub async fn debug_start_async(self_arc:Arc<Mutex<Self>>, channels_to_join: Vec<UserLogin>) {
        let h = tokio::task::spawn(async move {
            loop {
                println!("stuff!");
                sleep(Duration::from_millis(20));
            }

        });
        h.await;
    }


    // Init
    //
    pub async fn initialize(self_arc:Arc<Mutex<Self>>, channels_to_join: Vec<UserLogin>) {


        //let listen_future = Self::begin_continuous_listen(self_arc.clone(), &mut irc_listener);

        let self_arc1 = self_arc.clone();
        let self_arc2 = self_arc.clone();

        let mut irc_listener;
        {
            let local_arc = self_arc;

            let mut local_mutex = local_arc.deref().lock().unwrap();
            let mut self_ref = local_mutex;

            irc_listener = SyncableClient::new(self_ref.irc_url.clone());

            self_ref.authenticate_user(&mut irc_listener).await;
            for channel in channels_to_join {
                self_ref.join_channel(&mut irc_listener, channel);
            }
        }


        /*tokio::task::spawn( async move {
            loop {
                println!("1");
                delay_for(Duration::from_millis(10)).await;
            }
        });
        tokio::task::spawn( async move {
            loop {
                println!("2");
                delay_for(Duration::from_millis(10)).await;
            }
        });*/


        /*let listen_thread = std::thread::spawn(move || {
            println!("start listen");

            let self_arc = self_arc1.clone();

            let mut irc_listener = irc_listener;

            //delay_for(Duration::from_millis(2)).await;

            sleep(Duration::from_millis(2));

            loop {
                println!("lsiten outer");

                {
                    let self_mutex = self_arc1.deref().lock().unwrap();
                    IrcChatSession::listen(self_arc.clone(), &mut irc_listener);
                }

                //delay_for(Duration::from_millis(2)).await;
                sleep(Duration::from_millis(2));

            }
        });*/

        // listen thread
        let listen_handle = tokio::task::spawn_blocking(move || {

            println!("start listen");

            let self_arc = self_arc1.clone();

            let mut irc_listener = irc_listener;
            //irc_listener.set_nonblocking(true);

            sleep(Duration::from_millis(2));
            //delay_for(Duration::from_millis(2)).await;

            loop {
                println!("lsiten outer");


                IrcChatSession::listen(self_arc.clone(), &mut irc_listener);


                println!("Listen thread sleeping for 2ms");
                sleep(Duration::from_millis(2));
                //delay_for(Duration::from_millis(2)).await;
            }
        });

        //listen_thread.join();

        let tick_handle =

        // tick thread
        tokio::task::spawn(async move {

            println!("start tick");

            let self_arc = self_arc2.clone();

            let mut tick_rate = 0; {
                let self_mutex = self_arc.deref().lock().unwrap();
                tick_rate = self_mutex.tick_rate_ms.clone();
            }

            delay_for(Duration::from_millis(2)).await;

            loop {

                println!("outer tick");

                IrcChatSession::tick(self_arc.clone()).await;


                println!("delaying tick for {}ms", tick_rate);

                delay_for(Duration::from_millis(tick_rate)).await;
                //std::thread::sleep(Duration::from_millis(tick_rate - (begin_instant.elapsed().as_millis() as u64)));
            }
        });



        let (listen_result, tick_result) =  futures::future::join(listen_handle, tick_handle).await;

        match listen_result {
            Ok(_) => {
                println!("No errors from listen...");
            }
            Err(e) => {
                println!("Listen ERROR: {}", e);
            }
        }


        match tick_result {
            Ok(_) => {
                println!("No errors from tick...");
            }
            Err(e) => {
                println!("Tick ERROR: {}", e);
            }
        }
    }
    //
    async fn authenticate_user(&mut self, irc_dispatcher:&mut SyncableClient) {
        self.send_string(irc_dispatcher, format!("{0}{1}", IRC_TOKEN_PREFIX, self.user_token.get_oauth_signature().get_value()));
        self.send_string(irc_dispatcher, format!("{0}{1}", IRC_USERNAME_PREFIX, self.client_user.get_login().get_value()));
    }
    //
    fn listen(self_arc:Arc<Mutex<Self>>, irc_listener:&mut SyncableClient) {

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
    //

    async fn tick(self_arc:Arc<Mutex<Self>>/*, _irc_dispatcher:&mut Client<TcpStream>, reqwest_client:&reqwest::Client*/) {
        println!("IRC Tick.");


        let reqwest_client = reqwest::Client::builder().build().unwrap(); // TODO make syncable to prevent locking self's mutex

        let mut client_user:UserLogin;
        {
            let self_mutex = self_arc.deref();
            client_user = self_mutex.lock().unwrap().client_user.get_login();
        }
        let chatter_data = ChatterData::from_channel(&reqwest_client, client_user).await;

        for viewer in chatter_data.get_all_viewers(true, true) {
            println!("Tick sees viewer: {}", viewer.get_value());
        }
    }

    fn send_string(&self, irc_dispatcher:&mut SyncableClient, data_to_send: String) {
        irc_dispatcher.send_message(&Message::text(data_to_send.clone())).unwrap();

        if !data_to_send.contains("PASS") {
            println!("SENT: {}", data_to_send);
        }
    }

    fn register_received_data(&self, irc_dispatcher:&mut SyncableClient, received_data: &str) {
        const NEW_LINE: &str = "\r\n";

        for line in received_data.split(NEW_LINE) {
            if line.is_empty() { continue; }

            self.logger.write_line(format!("RCVD: {0}", line));

            self.process_response(irc_dispatcher, line.to_string());
        }
    }

    pub fn process_response (&self, irc_dispatcher:&mut SyncableClient, response:String) {
        let mut context = ResponseContext::new(self.client_user.clone(), response.to_string());
        if !self.message_parser.process_response(&mut context, self.logger.clone()) {
            self.logger.write_line(format!("IRC PARSER FAILED TO READ LINE: {0}", response));
        } else {
            for response_to_send in context.get_responses_to_reply_with() {
                self.send_string(irc_dispatcher, response_to_send);
            }
        }
    }

    pub fn join_channel(&mut self, irc_dispatcher:&mut SyncableClient, user_login: UserLogin) {
        const JOIN_PREFIX: &str = "JOIN #";

        self.send_string(irc_dispatcher, format!("{0}{1}", JOIN_PREFIX, user_login.get_value()));
    }
}