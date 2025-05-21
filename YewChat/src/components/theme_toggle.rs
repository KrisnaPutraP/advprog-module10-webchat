use crate::ThemeContext;
use yew::prelude::*;

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let theme_ctx = use_context::<ThemeContext>().expect("no theme context found");
    
    let onclick = {
        let theme_ctx = theme_ctx.clone();
        
        Callback::from(move |_| {
            let is_dark = !*theme_ctx.dark_mode.borrow();
            *theme_ctx.dark_mode.borrow_mut() = is_dark;
            
            // Update HTML class for dark mode
            let window = web_sys::window().expect("no global window exists");
            let document = window.document().expect("no document exists");
            let html = document.document_element().expect("no document element");
            
            if is_dark {
                // Add dark class
                let current_class = html.get_attribute("class").unwrap_or_default();
                if !current_class.contains("dark") {
                    html.set_attribute("class", &format!("{} dark", current_class.trim()))
                        .expect("failed to set class attribute");
                }
                // Save preference to local storage
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("theme", "dark");
                }
            } else {
                // Remove dark class
                let current_class = html.get_attribute("class").unwrap_or_default();
                let new_class = current_class
                    .split_whitespace()
                    .filter(|&c| c != "dark")
                    .collect::<Vec<&str>>()
                    .join(" ");
                html.set_attribute("class", &new_class)
                    .expect("failed to set class attribute");
                // Save preference to local storage
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("theme", "light");
                }
            }
        })
    };

    let is_dark = *theme_ctx.dark_mode.borrow();

    html! {
        <button 
            {onclick}
            class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
            aria-label="Toggle dark mode"
        >
            if is_dark {
                <svg class="w-6 h-6 text-yellow-400" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                    <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd"></path>
                </svg>
            } else {
                <svg class="w-6 h-6 text-gray-700" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                    <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path>
                </svg>
            }
        </button>
    }
}
