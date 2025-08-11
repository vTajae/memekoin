use leptos::prelude::*;
use crate::context::auth::use_auth;

#[component]
pub fn LogoutButton(
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] text: Option<&'static str>,
) -> impl IntoView {
    let auth = use_auth();
    let logout_text = text.unwrap_or("Sign out");
    let button_classes = format!(
        "inline-flex items-center px-3 py-2 text-sm text-gray-700 hover:text-gray-900 hover:bg-gray-100 rounded-md transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed {}",
        class.unwrap_or("")
    );

    view! {
        <button
            class=button_classes
            on:click=move |_| {
                auth.logout.run(());
            }
        >
            // Sign out icon
            <svg 
                class="w-4 h-4 mr-2" 
                fill="none" 
                stroke="currentColor" 
                viewBox="0 0 24 24"
            >
                <path 
                    stroke-linecap="round" 
                    stroke-linejoin="round" 
                    stroke-width="2" 
                    d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
                />
            </svg>
            
            {logout_text}
        </button>
    }
}