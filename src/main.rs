extern crate async_trait;
#[macro_use]
extern crate colour;
#[macro_use]
extern crate kubes_std_lib;
extern crate tokio;

use crate::irc_chat::commands::{
    get_user_commands_including_alternates, send_message_from_user_format, ChatCommandKey,
    CommandContext,
};
use crate::irc_chat::parsers::default_irc_message_parser::UserNativeCommandsMap;
use crate::irc_chat::response_context::ResponseContext;
use crate::irc_chat::twitch_message_type::TwitchIrcMessageType;
use crate::irc_chat::twitch_user_message::TwitchIrcUserMessage;
use crate::save_data::default::custom_commands_save_data::CustomCommandsSaveData;
use crate::user::user_properties::{ChannelData, UserLogin};
use crate::{main_tick_data::TickData, user::user_properties::ChannelId};
use irc_chat::channel_chatter_data::ChatterData;
pub use kubes_std_lib::logging::{DefaultLogger, Logger};
use kubes_std_lib::text::impl_to_string::{begins_with, remove_within};
use kubes_web_lib::error::send_error::BoxSendError;
use kubes_web_lib::error::{send_error, send_result, SendResult};
pub use kubes_web_lib::web_request::{is_html, is_json};
use kubes_web_lib::web_socket::{Session, TOnDataReceivedFuture};
use oauth::has_oauth_signature::HasOauthSignature;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use secrets::{CLIENT_ID, CLIENT_SECRET};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::{convert::TryFrom, error::Error, str::FromStr, sync::Arc, time::Instant};
use tokio::sync::Mutex;
use tokio::time::{delay_for as sleep, Duration};
use user::{oauth_token::OauthToken as UserOauthToken, user_data::UserData};
use websocket::websocket_base::ws::dataframe::DataFrame;
use websocket::{
    stream::sync::{TcpStream, TlsStream},
    url::Url,
    OwnedMessage, WebSocketError,
};

pub mod credentials;
pub mod irc_chat;
pub mod main_tick_data;
pub mod oauth;
pub mod save_data;
pub mod secrets;
pub mod user;
pub mod web_requests;

pub struct BotState {
    pub irc_channel: ChannelData,
    pub bot_client_channel: ChannelData,
}

const TICK_RATE: u64 = 1000;

fn get_pubsub_secure_url() -> Url {
    Url::from_str("wss://pubsub-edge.twitch.tv").unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (token, user) = init_token_and_user(&DefaultLogger {}).await?;

    start_chat_session(token.clone(), user.clone()).await?;

    let channel = ChannelData {
        owner_data: user.clone(),
    };
    start_pubsub_session(token.clone(), channel).await?;

    tick(token, user, TICK_RATE).await
}

#[derive(Deserialize, Serialize)]
struct ClientToServerRequest {
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<String>,
    data: ClientToServerRequestData,
}

#[derive(Deserialize, Serialize)]
struct ClientToServerRequestData {
    topics: Vec<String>,
    auth_token: String,
}

/// Interacts with Twitch Events such as subs and channel points spending
async fn start_pubsub_session(
    token: UserOauthToken,
    user: ChannelData,
) -> Result<(), Box<dyn Error>> {
    let start_request = serde_json::to_string(&ClientToServerRequest {
        r#type: "LISTEN".to_string(),
        nonce: Some("333".to_string()),
        data: ClientToServerRequestData {
            topics: vec![format!(
                "channel-points-channel-v1.{}",
                user.owner_data.clone().get_user_id().get_value()
            )],
            auth_token: token.clone().get_oauth_signature().get_value(),
        },
    })?;

    Session::<BotState>::new(
        get_pubsub_secure_url().into_string(),
        2,
        |session| {
            session.send_string(start_request.clone());
            Ok(())
        },
        Arc::new(Box::pin(|session, incoming_message| {
            return Box::pin(async move {
                let mut deconstructing_response = incoming_message.clone();
                const TMI_TWITCH: &str = ":tmi.twitch.tv ";

                if begins_with(&incoming_message, TMI_TWITCH) {
                    deconstructing_response = remove_within(&deconstructing_response, TMI_TWITCH);

                    process_twitch_irc_code(session, &deconstructing_response[..3])?;
                    return Ok(());
                }

                if begins_with(&incoming_message, "PING ") {
                    // SEND String::from("PONG :tmi.twitch.tv")
                    session.lock().await.send_string("PONG :tmi.twitch.tv");

                    return Ok(());
                }

                let context = ResponseContext {
                    message: incoming_message,
                };

                match decipher_response_message(session.clone(), context.clone()).await {
                    Err(e) => println!(
                        "IF THIS ISNT A USER MESSAGE.... WTF IS IT?? {}",
                        e.to_string()
                    ),
                    Ok(message) => {
                        match message {
                            TwitchIrcMessageType::Client => {
                                //println!("Client message...");
                            }
                            TwitchIrcMessageType::Message(message) => {
                                if begins_with(&message.get_message_body(), "!") {
                                    let a = try_execute_command(session, message.clone(), context);
                                    a.await?;
                                } else {
                                    println!(
                                        "{}'s channel: {} said {}",
                                        message.get_target_channel().get_value(),
                                        message.get_speaker().get_value(),
                                        message.get_message_body()
                                    );
                                }
                            }
                            TwitchIrcMessageType::JoiningChannel { joiner, channel } => {
                                println!(
                                    "({0}'s channel): {1} has JOINED the channel!",
                                    channel.get_value(),
                                    joiner.get_value()
                                );
                            }
                            TwitchIrcMessageType::LeavingChannel { leaver, channel } => {
                                println!(
                                    "({0}'s channel): {1} has LEFT the channel!",
                                    channel.get_value(),
                                    leaver.get_value()
                                );
                            }
                        }
                    }
                };

                Ok(())
            });
        })),
        BotState {
            irc_channel: user.clone(),
            bot_client_channel: user,
        },
    )?
    .run()
    .unwrap();

    Ok(())
}

// TODO make sure responses to lines are sent
fn process_line(
    session: Arc<Mutex<Session<BotState>>>,
    incoming_message: String,
) -> TOnDataReceivedFuture {
    return Box::pin(async move {
        let mut deconstructing_response = incoming_message.clone();
        const TMI_TWITCH: &str = ":tmi.twitch.tv ";

        if begins_with(&incoming_message, TMI_TWITCH) {
            deconstructing_response = remove_within(&deconstructing_response, TMI_TWITCH);

            process_twitch_irc_code(session, &deconstructing_response[..3])?;
            return Ok(());
        }

        if begins_with(&incoming_message, "PING ") {
            // SEND String::from("PONG :tmi.twitch.tv")
            session.lock().await.send_string("PONG :tmi.twitch.tv");

            return Ok(());
        }

        let context = ResponseContext {
            message: incoming_message,
        };

        match decipher_response_message(session.clone(), context.clone()).await {
            Err(e) => println!(
                "IF THIS ISNT A USER MESSAGE.... WTF IS IT?? {}",
                e.to_string()
            ),
            Ok(message) => {
                match message {
                    TwitchIrcMessageType::Client => {
                        //println!("Client message...");
                    }
                    TwitchIrcMessageType::Message(message) => {
                        if begins_with(&message.get_message_body(), "!") {
                            let a = try_execute_command(session, message.clone(), context);
                            a.await?;
                        } else {
                            println!(
                                "{}'s channel: {} said {}",
                                message.get_target_channel().get_value(),
                                message.get_speaker().get_value(),
                                message.get_message_body()
                            );
                        }
                    }
                    TwitchIrcMessageType::JoiningChannel { joiner, channel } => {
                        println!(
                            "({0}'s channel): {1} has JOINED the channel!",
                            channel.get_value(),
                            joiner.get_value()
                        );
                    }
                    TwitchIrcMessageType::LeavingChannel { leaver, channel } => {
                        println!(
                            "({0}'s channel): {1} has LEFT the channel!",
                            channel.get_value(),
                            leaver.get_value()
                        );
                    }
                }
            }
        };

        Ok(())
    });
}

fn get_process_line_method(
    channel_user: UserData,
) -> Pin<Box<dyn Fn(Arc<Mutex<Session<BotState>>>, String) -> TOnDataReceivedFuture>> {
    return Box::pin(
        move |session: Arc<Mutex<Session<BotState>>>, incoming_message: String| {
            let mut deconstructing_response = incoming_message.clone();
            const TMI_TWITCH: &str = ":tmi.twitch.tv ";

            if begins_with(&incoming_message, TMI_TWITCH) {
                deconstructing_response = remove_within(&deconstructing_response, TMI_TWITCH);

                return Box::pin(async move {
                    process_twitch_irc_code(session, &deconstructing_response[..3])?;
                    Ok(())
                });
            }

            if begins_with(&incoming_message, "PING ") {
                return Box::pin(async move {
                    session.lock().await.send_string("PONG :tmi.twitch.tv");

                    Ok(())
                });
            }

            //return Box::pin(async move { Ok(()) });
            return Box::pin(async move {
                let context = ResponseContext {
                    message: incoming_message,
                };

                match decipher_response_message(session.clone(), context.clone()).await {
                    Err(e) => println!(
                        "IF THIS ISNT A USER MESSAGE.... WTF IS IT?? {}",
                        e.to_string()
                    ),
                    Ok(message) => {
                        match message {
                            TwitchIrcMessageType::Client => {
                                //println!("Client message...");
                            }
                            TwitchIrcMessageType::Message(message) => {
                                if begins_with(&message.get_message_body(), "!") {
                                    let a = try_execute_command(session, message.clone(), context);
                                    a.await?;
                                } else {
                                    println!(
                                        "{}'s channel: {} said {}",
                                        message.get_target_channel().get_value(),
                                        message.get_speaker().get_value(),
                                        message.get_message_body()
                                    );
                                }
                            }
                            TwitchIrcMessageType::JoiningChannel { joiner, channel } => {
                                println!(
                                    "({0}'s channel): {1} has JOINED the channel!",
                                    channel.get_value(),
                                    joiner.get_value()
                                );
                            }
                            TwitchIrcMessageType::LeavingChannel { leaver, channel } => {
                                println!(
                                    "({0}'s channel): {1} has LEFT the channel!",
                                    channel.get_value(),
                                    leaver.get_value()
                                );
                            }
                        }
                    }
                };

                Ok(())
            });
        },
    );
}
//
fn process_twitch_irc_code(session: Arc<Mutex<Session<BotState>>>, code: &str) -> SendResult<()> {
    match code {
        "001" => { /*Welcome, GLHF*/ }
        "002" => { /*Your host is tmi.twitch.tv*/ }
        "003" => { /*This server is rather new*/ }
        "004" => { /*-*/ }
        "372" => { /*You are in a maze of twisty passages, all alike.*/ }
        "375" => { /*-*/ }
        "376" => { /*>*/ }
        "421" => { /* Unknown command */ }
        _ => {
            println!("IRC parser Not aware of Twitch-code {0}.", code.to_string());
        }
    }
    Ok(())
}
//
async fn decipher_response_message(
    session: Arc<Mutex<Session<BotState>>>,
    context: ResponseContext,
) -> SendResult<TwitchIrcMessageType> {
    let deconstructing_response;

    {
        if !begins_with(&context.message, ":") {
            return Err(send_error::boxed(""));
        }

        deconstructing_response = {
            let initial = context.message[1..].to_string();
            let mut temp = String::new();

            // remove duplicate whitespace
            let mut previous_character = ' ';
            for character in initial.chars() {
                if character == ' ' && previous_character == ' ' {
                    continue;
                }

                temp = format!("{0}{1}", temp, character);
                previous_character = character;
            }

            temp
        };
    }

    let mut first_username_split = deconstructing_response.split("!");

    let potential_username = {
        match first_username_split.next() {
            None => return Err(send_error::boxed("")),
            Some(r) => r,
        }
    };
    let username_duplicate = first_username_split.next(); //?.split("@").next()?;
    if username_duplicate == None {
        let mut client_username_split = deconstructing_response.split(".");

        {
            let channel_user_data = session.lock().await.state.irc_channel.owner_data.clone();

            if send_result::from_option(client_username_split.next())?.to_string()
                != channel_user_data.get_login().get_value()
            {
                return Err(send_error::boxed(""));
            }
        };

        send_result::from_option(client_username_split.next())?; // tmi
        send_result::from_option(client_username_split.next())?; // twitch

        // there SHOULDNT be any more periods....

        let response_after_client_name = {
            let temp = send_result::from_option(client_username_split.next())?;
            if temp.len() < 4 {
                return Err(send_error::boxed(""));
            }

            temp[3..].to_string() // remove 'tv '
        };

        let mut response_whitespace_split = response_after_client_name.split(" ");

        println!(
            "{}",
            match send_result::from_option(response_whitespace_split.next())? {
                "353" => {
                    "Is this message only when the client joins? or when anyone joins a channel?"
                }
                "366" => {
                    "End of names list.... (list only shows client's name atm) is this only after the client joins a channel?"
                }
                _ => {
                    "??? Client Message"
                }
            }
        );

        return Ok(TwitchIrcMessageType::Client);
    }

    let username_duplicate = send_result::from_option(
        send_result::from_option(username_duplicate)?
            .split("@")
            .next(),
    )?;
    // check if not a usual message (could begin with [client_user].tmi.twitch.tv)
    if potential_username != username_duplicate {
        return Err(send_error::boxed(""));
    }

    let username = UserLogin::from(potential_username.to_string());

    let mut whitespace_split = deconstructing_response.split(" ");
    send_result::from_option(whitespace_split.next())?;
    let message_type = send_result::from_option(whitespace_split.next())?;

    let channel_name = {
        let dirty_channel_name = send_result::from_option(whitespace_split.next())?;
        if dirty_channel_name.len() < 2 {
            return Err(send_error::boxed(""));
        }
        UserLogin::from(dirty_channel_name[1..].to_string()) // remove pound symbol
    };

    match message_type {
        "PRIVMSG" => {
            let message = {
                let mut potential_message =
                    send_result::from_option(whitespace_split.next())?.to_string();
                if potential_message.len() < 2 {
                    return Err(send_error::boxed(
                        "Twitch PRIVMSG has an empty body or an invalid format.".to_string(),
                    ));
                }

                while let Some(next_word) = whitespace_split.next() {
                    potential_message = format!("{0} {1}", potential_message, next_word);
                }

                potential_message[1..].to_string() // remove first space
            };

            Ok(TwitchIrcMessageType::Message(TwitchIrcUserMessage::new(
                username,
                message,
                channel_name,
            )))
        }
        "JOIN" => Ok(TwitchIrcMessageType::JoiningChannel {
            joiner: username,
            channel: channel_name,
        }),
        _ => {
            println!("Could not register IRC message type: {}", message_type);
            return Err(send_error::boxed(""));
        }
    }
}
//
async fn try_execute_command(
    session_arc: Arc<Mutex<Session<BotState>>>,
    message: TwitchIrcUserMessage,
    context: CommandContext,
) -> SendResult<()> {
    let session_arc_first = session_arc.clone();
    let channel_id = ChannelId::from(
        session_arc_first
            .lock()
            .await
            .state
            .irc_channel
            .owner_data
            .get_user_id(),
    );

    verify_message(&message)?;

    let message_body = message.get_message_body();

    // for retrieving command and args
    let mut whitespace_split = message_body[1..].split(" ");

    let command_string = send_result::from_option(whitespace_split.next())?.to_lowercase(); // temp to maintain lifetime
    let command = ChatCommandKey::from(command_string);

    let command_args = {
        let mut temp = vec![];
        while let Some(arg) = whitespace_split.next() {
            temp.push(arg.to_string());
        }
        temp
    };

    // if not a native command, try running as a custom command
    if !execute_native_command(
        session_arc.clone(),
        command.clone(),
        command_args.clone(),
        channel_id.clone(),
        message.clone(),
    )
    .await?
    {
        execute_custom_command(
            session_arc,
            command,
            command_args,
            channel_id,
            message,
            context,
        )
        .await?;
    }

    Ok(())
}
//
fn verify_message(message: &TwitchIrcUserMessage) -> SendResult<()> {
    if send_result::from_option(message.get_message_body().chars().next())? != '!' // commands begin with "!"
        || message.get_message_body().len() == 1
    {
        return Err(send_error::boxed("Twitch - invalid message."));
    }
    Ok(())
}
//
async fn execute_native_command(
    session: Arc<Mutex<Session<BotState>>>,
    command: ChatCommandKey,
    command_args: Vec<String>,
    _channel: ChannelId,
    message: TwitchIrcUserMessage,
) -> SendResult<bool> {
    let (user_commands, commands_alt_keys) = get_user_commands_including_alternates();

    if let Some(command_func) = user_commands.get(&command) {
        command_func
            .lock()
            .await
            .run(session, message.clone(), command_args)
            .await?;

        println!(
            "{0} triggered !{1}.",
            message.get_speaker().get_value(),
            command.get_value()
        );

        return Ok(true);
    }

    let user_commands_alternate_keywords: UserNativeCommandsMap;

    if let Some(command_func) = commands_alt_keys.get(&command) {
        command_func
            .lock()
            .await
            .run(session, message.clone(), command_args)
            .await?;

        println!(
            "{0} triggered !{1}.",
            message.get_speaker().get_value(),
            command.get_value()
        );

        return Ok(true);
    }

    Ok(false)
}
//
async fn execute_custom_command(
    session: Arc<Mutex<Session<BotState>>>,
    command: ChatCommandKey,
    command_args: Vec<String>,
    channel: ChannelId,
    message: TwitchIrcUserMessage,
    context_mutex: CommandContext,
) -> Result<bool, BoxSendError> {
    let custom_commands =
        send_result::from_dyn(CustomCommandsSaveData::load_or_default(channel))?.get_commands();

    // command exists?
    if !(custom_commands.contains_key(&command)) {
        return Ok(false);
    }

    if command_args.len() > 0 {
        println!("Custom command triggered with extra arguments given.");
    }

    // Add custom command's text to respond with
    {
        let message_body = send_result::from_option(custom_commands.get(&command))?.clone();
        session
            .lock()
            .await
            .send_string(send_message_from_user_format(
                message.get_target_channel().clone(),
                message_body,
            ))
    }

    Ok(true)
}

/// Interacts with Twitch IRC chat in a designated channel
async fn start_chat_session(token: UserOauthToken, user: UserData) -> Result<(), Box<dyn Error>> {
    unimplemented!(); /*
                      let chat_url = websocket::url::Url::from_str("wss://irc-ws.chat.twitch.tv:443")?;
                      // create Chat-IRC
                      let chat_session = WebSocketSession::new(
                          DefaultMessageParser::new(user.clone(), token.clone()),
                          chat_url,
                      );
                      //
                      // run Chat-IRC
                      let chat_irc_arc = Arc::new(tokio::sync::Mutex::new(chat_session));
                      let token_temp = token.clone();
                      let user_temp = user.clone();
                      let on_chat_start =
                          move |session: &mut WebSocketSession<DefaultMessageParser>,
                                listener: &mut websocket::sync::Client<TlsStream<TcpStream>>| {
                              // authenticate user (login)
                              session.send_string(
                                  listener,
                                  format!(
                                      "PASS oauth:{}",
                                      token_temp.clone().get_oauth_signature().get_value()
                                  ),
                              );
                              session.send_string(
                                  listener,
                                  format!("NICK {}", user_temp.clone().get_login().get_value()),
                              );
                              session.send_string(
                                  listener,
                                  format!("JOIN #{}", user_temp.get_login().get_value()),
                              );
                          };
                      WebSocketSession::initialize(chat_irc_arc, on_chat_start).await;
                      Ok(())*/
}

/// Get OAuth token and the logged in client_user's info
async fn init_token_and_user<TLogger>(
    logger: &TLogger,
) -> Result<(UserOauthToken, UserData), Box<dyn Error>>
where
    TLogger: Logger,
{
    let client = Client::builder().build()?;

    // get user token
    let token = UserOauthToken::request(&client, CLIENT_ID.clone(), CLIENT_SECRET, logger).await?;

    // get logged in user's info
    let user_data =
        user::user_data::UserData::get_from_bearer_token(&client, token.clone(), logger).await?;

    Ok((token, user_data))
}

async fn tick(token: UserOauthToken, user: UserData, tick_rate: u64) -> Result<(), Box<dyn Error>> {
    let reqwest_client = reqwest::Client::builder().build()?;
    let mut tick_data = Default::default();

    sleep(Duration::from_millis(tick_rate)).await; // don't trigger before WebSockets start

    loop {
        let before_tick_instant = Instant::now();

        tick_routine(&reqwest_client, user.clone(), token.clone(), &mut tick_data).await?;

        let tick_elapsed_time = match u64::try_from(before_tick_instant.elapsed().as_millis()) {
            Ok(elapsed_time_since_tick) => {
                if elapsed_time_since_tick >= tick_rate {
                    tick_rate
                } else {
                    elapsed_time_since_tick
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                0 as u64
            }
        };
        sleep(Duration::from_millis(tick_rate - tick_elapsed_time)).await;
    }
}
//
async fn tick_routine(
    reqwest_client: &reqwest::Client,
    client_user: UserData,
    bearer_token: UserOauthToken,
    tick_data: &mut TickData,
) -> Result<(), Box<dyn Error>> {
    // Headers
    let mut header_map = HeaderMap::new();
    // Client Header
    let client_header = bearer_token.get_client_id().get_header()?;
    let header_name = HeaderName::from_str(client_header.key.as_str())?;
    header_map.append(
        header_name,
        HeaderValue::from_str(bearer_token.get_client_id().value.as_str())?,
    );
    // Bearer Header
    let bearer_header = bearer_token.get_oauth_bearer_header();
    let header_name = HeaderName::from_str(bearer_header.key.as_str())?;
    header_map.append(header_name, bearer_header.value);

    //
    let chatter_data = ChatterData::from_channel(&reqwest_client, client_user.get_login()).await?;
    let viewers = chatter_data.get_all_viewers(true, true);
    let viewer_datas =
        UserData::get_from_usernames(reqwest_client, viewers, &DefaultLogger {}, header_map)
            .await?;

    tick_data.tick_on_users(
        client_user.get_user_id(),
        viewer_datas
            .iter()
            .map(|d| d.clone().get_user_id())
            .collect(),
    )?;
    Ok(())
}
