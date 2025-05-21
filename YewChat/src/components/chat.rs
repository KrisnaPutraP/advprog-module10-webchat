use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};
use crate::components::theme_toggle::ThemeToggle;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let (user, _) = ctx.link().context::<User>(Callback::noop()).expect("context to be set");
        let current_username = user.username.borrow().clone();

        html! {
            <div class="flex flex-col md:flex-row h-screen bg-gray-100 dark:bg-gray-900 transition-colors duration-200">
                <div class="flex-none w-full md:w-80 h-auto md:h-screen bg-white dark:bg-gray-800 shadow-lg overflow-y-auto">
                    <div class="sticky top-0 bg-violet-600 dark:bg-violet-800 text-white p-4 font-bold text-lg flex justify-between items-center">
                        <div class="flex items-center">
                            <svg class="w-6 h-6 mr-2" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z"></path>
                            </svg>
                            {format!("Online Users ({})", self.users.len())}
                        </div>
                        <ThemeToggle />
                    </div>
                    <div class="p-3 space-y-3">
                    {
                        if self.users.is_empty() {
                            html! {
                                <div class="flex items-center justify-center h-20 text-gray-500 dark:text-gray-400 italic">
                                    {"No users online"}
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                    {
                        self.users.clone().iter().map(|u| {
                            let is_current_user = u.name == current_username;
                            html!{
                                <div class={format!("flex items-center p-3 rounded-lg {} {}", 
                                    if is_current_user { 
                                        "bg-violet-100 dark:bg-violet-900 border-l-4 border-violet-500" 
                                    } else { 
                                        "bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700" 
                                    },
                                    "transition-all duration-200 transform hover:translate-x-1 shadow-sm"
                                )}>
                                    <div class="relative">
                                        <img class="w-12 h-12 rounded-full border-2 border-gray-200 dark:border-gray-600" src={u.avatar.clone()} alt="avatar"/>
                                        <div class="absolute bottom-0 right-0 w-3.5 h-3.5 bg-green-500 border-2 border-white dark:border-gray-700 rounded-full"></div>
                                    </div>
                                    <div class="flex-grow ml-3">
                                        <div class="flex justify-between items-center">
                                            <div class={format!("font-medium {}", 
                                                if is_current_user { 
                                                    "text-violet-700 dark:text-violet-300" 
                                                } else { 
                                                    "text-gray-700 dark:text-gray-300" 
                                                }
                                            )}>
                                                {u.name.clone()}{if is_current_user { " (You)" } else { "" }}
                                            </div>
                                        </div>
                                        <div class="text-xs text-gray-500 dark:text-gray-400">
                                            {"Active now"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    </div>
                </div>
                
                <div class="grow h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
                    <div class="w-full h-16 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-sm flex items-center px-5">
                        <div class="flex items-center">
                            <svg class="w-6 h-6 text-violet-600 dark:text-violet-400" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd" d="M18 10c0 3.866-3.582 7-8 7a8.841 8.841 0 01-4.083-.98L2 17l1.338-3.123C2.493 12.767 2 11.434 2 10c0-3.866 3.582-7 8-7s8 3.134 8 7zM7 9H5v2h2V9zm8 0h-2v2h2V9zM9 9h2v2H9V9z" clip-rule="evenodd"></path>
                            </svg>
                            <h1 class="ml-2 text-xl font-semibold text-gray-800 dark:text-gray-100">{"YewChat Room"}</h1>
                        </div>
                    </div>
                    <div class="w-full grow overflow-auto p-6 space-y-4">
                        {
                            if self.messages.is_empty() {
                                html! {
                                    <div class="flex flex-col items-center justify-center h-full text-gray-500 dark:text-gray-400">
                                        <svg class="w-16 h-16 mb-4 text-gray-300 dark:text-gray-600" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                            <path fill-rule="evenodd" d="M18 10c0 3.866-3.582 7-8 7a8.841 8.841 8.841 0 01-4.083-.98L2 17l1.338-3.123C2.493 12.767 2 11.434 2 10c0-3.866 3.582-7 8-7s8 3.134 8 7zM7 9H5v2h2V9zm8 0h-2v2h2V9zM9 9h2v2H9V9z" clip-rule="evenodd"></path>
                                        </svg>
                                        <p class="text-lg">{"No messages yet"}</p>
                                        <p class="text-sm mt-2">{"Start the conversation by sending a message below!"}</p>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        {
                            self.messages.iter().map(|m| {
                                let user_profile = self.users.iter().find(|u| u.name == m.from);
                                let is_current_user = m.from == current_username;
                                let avatar = user_profile.map_or_else(
                                    || format!("https://avatars.dicebear.com/api/adventurer-neutral/{}.svg", m.from),
                                    |u| u.avatar.clone()
                                );

                                html!{
                                    <div class={format!("flex {}", if is_current_user { "justify-end" } else { "justify-start" })}>
                                        <div class={format!("flex items-end max-w-[80%] md:max-w-[60%] {}", 
                                            if is_current_user { "flex-row-reverse" } else { "flex-row" }
                                        )}>
                                            <img class={format!("w-8 h-8 rounded-full {}", if is_current_user { "ml-2" } else { "mr-2" })} 
                                                src={avatar} alt="avatar"/>
                                            <div class={format!("px-4 py-3 rounded-t-lg {} space-y-1 shadow-sm", 
                                                if is_current_user { 
                                                    "bg-violet-600 dark:bg-violet-700 text-white rounded-bl-lg rounded-br-none" 
                                                } else { 
                                                    "bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded-br-lg rounded-bl-none" 
                                                }
                                            )}>
                                                <div class="text-xs font-medium">
                                                    {if is_current_user { "You" } else { &m.from }}
                                                </div>
                                                <div class={if is_current_user { "text-violet-100" } else { "text-gray-700 dark:text-gray-300" }}>
                                                    if m.message.ends_with(".gif") {
                                                        <img class="mt-2 rounded-lg max-w-full" src={m.message.clone()}/>
                                                    } else {
                                                        <p class="break-words">{m.message.clone()}</p>
                                                    }
                                                </div>
                                                <div class={format!("text-[10px] {}", 
                                                    if is_current_user { "text-violet-200" } else { "text-gray-400 dark:text-gray-500" }
                                                )}>
                                                    {"just now"}
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="w-full bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 p-3 flex items-center">
                        <input 
                            ref={self.chat_input.clone()} 
                            type="text" 
                            placeholder="Type a message..." 
                            class="block w-full py-3 px-4 bg-gray-100 dark:bg-gray-700 rounded-full outline-none focus:ring-2 focus:ring-violet-500 focus:bg-white dark:focus:bg-gray-600 transition-all text-gray-800 dark:text-gray-200 placeholder-gray-500 dark:placeholder-gray-400" 
                            name="message" 
                            required=true 
                        />
                        <button 
                            onclick={submit} 
                            class="ml-3 p-3 bg-violet-600 hover:bg-violet-700 dark:bg-violet-700 dark:hover:bg-violet-800 transition-colors w-12 h-12 rounded-full flex justify-center items-center text-white shadow-lg hover:shadow-violet-300/50 dark:hover:shadow-violet-900/50"
                        >
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 fill-current">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
