#![recursion_limit = "512"]

mod components;
mod services;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use components::chat::Chat;
use components::login::Login;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/chat")]
    Chat,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub type User = Rc<UserInner>;
pub type ThemeContext = Rc<ThemeContextInner>;

#[derive(Debug, PartialEq)]
pub struct UserInner {
    pub username: RefCell<String>,
}

#[derive(Debug, PartialEq)]
pub struct ThemeContextInner {
    pub dark_mode: RefCell<bool>,
}

#[function_component(Main)]
fn main() -> Html {
    let user_ctx = use_state(|| {
        Rc::new(UserInner {
            username: RefCell::new("initial".into()),
        })
    });

    // Create theme context with default as light mode
    let theme_ctx = use_state(|| {
        Rc::new(ThemeContextInner {
            dark_mode: RefCell::new(false),
        })
    });

    // Check if user has saved theme preference
    {
        let theme_ctx_clone = theme_ctx.clone();
        use_effect_with_deps(
            move |_| {
                let window = web_sys::window().expect("no global window exists");
                let storage = window.local_storage().expect("local storage not available").unwrap();
                
                if let Ok(Some(theme)) = storage.get_item("theme") {
                    if theme == "dark" {
                        *theme_ctx_clone.dark_mode.borrow_mut() = true;
                        let document = window.document().expect("no document exists");
                        let html = document.document_element().expect("no document element");
                        
                        // Add dark class
                        let current_class = html.get_attribute("class").unwrap_or_default();
                        if !current_class.contains("dark") {
                            html.set_attribute("class", &format!("{} dark", current_class.trim()))
                                .expect("failed to set class attribute");
                        }
                    }
                }
                || {}
            },
            (),
        );
    }

    let theme_class = if *theme_ctx.dark_mode.borrow() { "dark" } else { "" };

    html! {
        <ContextProvider<User> context={(*user_ctx).clone()}>
            <ContextProvider<ThemeContext> context={(*theme_ctx).clone()}>
                <div class={classes!("transition-colors", "duration-200", theme_class)}>
                    <BrowserRouter>
                        <div class="flex w-screen h-screen bg-gray-50 dark:bg-gray-900">
                            <div class="w-full h-full overflow-hidden">
                                <Switch<Route> render={Switch::render(switch)}/>
                            </div>
                        </div>
                    </BrowserRouter>
                </div>
            </ContextProvider<ThemeContext>>
        </ContextProvider<User>>
    }
}

fn switch(selected_route: &Route) -> Html {
    match selected_route {
        Route::Login => html! {<Login />},
        Route::Chat => html! {<Chat/>},
        Route::NotFound => html! {
            <div class="flex items-center justify-center h-screen bg-gray-100 dark:bg-gray-800">
                <div class="text-center">
                    <h1 class="text-6xl font-bold text-gray-800 dark:text-gray-100">{"404"}</h1>
                    <p class="text-xl text-gray-600 dark:text-gray-300 mt-4">{"Page not found"}</p>
                    <Link<Route> to={Route::Login} classes="mt-6 inline-block px-6 py-3 bg-violet-600 text-white rounded-lg hover:bg-violet-700 transition">
                        {"Back to Login"}
                    </Link<Route>>
                </div>
            </div>
        },
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}
