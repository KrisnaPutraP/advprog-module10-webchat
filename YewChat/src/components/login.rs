use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;
use crate::components::theme_toggle::ThemeToggle;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="flex flex-col items-center justify-center min-h-screen bg-gradient-to-r from-violet-500 to-purple-700 dark:from-violet-900 dark:to-purple-950 transition-colors duration-200">
            <div class="absolute top-4 right-4">
                <ThemeToggle />
            </div>
            <div class="w-full max-w-md p-8 space-y-8 bg-white dark:bg-gray-800 rounded-lg shadow-lg transform transition duration-500 hover:scale-105">
                <div class="text-center">
                    <div class="flex justify-center">
                        <img src="https://cdn-icons-png.flaticon.com/512/134/134914.png" class="w-20 h-20 animate-bounce-slow" alt="Logo" />
                    </div>
                    <h2 class="mt-6 text-3xl font-extrabold text-gray-900 dark:text-white">{"Welcome to YewChat"}</h2>
                    <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">{"Connect with others using Rust & WebAssembly"}</p>
                </div>
                
                <div class="mt-8 space-y-6">
                    <div class="rounded-md shadow-sm">
                        <input 
                            {oninput} 
                            type="text"
                            placeholder="Enter your username" 
                            class="relative block w-full px-4 py-3 text-gray-900 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-violet-500 focus:border-transparent dark:bg-gray-700 dark:border-gray-600 dark:text-white dark:placeholder-gray-400" 
                        />
                    </div>
                    
                    <div>
                        <Link<Route> to={Route::Chat}>
                            <button 
                                {onclick} 
                                disabled={username.len()<1}
                                class={format!("group relative w-full flex justify-center py-3 px-4 border border-transparent text-sm font-medium rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500 {} {}", 
                                    if username.len() >= 1 {"bg-violet-600 hover:bg-violet-700"} else {"bg-violet-400 cursor-not-allowed"},
                                    if username.len() >= 1 {"animate-pulse-slow"} else {""}
                                )}
                            >
                                <span class="absolute left-0 inset-y-0 flex items-center pl-3">
                                    <svg class="h-5 w-5 text-violet-300 group-hover:text-violet-200" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M10.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L12.586 11H5a1 1 0 110-2h7.586l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd" />
                                    </svg>
                                </span>
                                {"Start Chatting"}
                            </button>
                        </Link<Route>>
                    </div>
                </div>
                
                <div class="mt-6 text-center text-sm text-gray-500 dark:text-gray-400">
                    {"Built with "}<span class="text-red-500">{"❤"}</span>{" using Rust + Yew by Krisna Putra Purnomo"}
                </div>
            </div>
        </div>
    }
}
