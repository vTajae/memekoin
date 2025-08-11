use leptos::prelude::*;
use crate::context::auth::{use_auth_state, use_current_user};
use crate::components::auth::login_button::LoginButton;
use crate::components::auth::logout_button::LogoutButton;
use crate::types::auth::AuthState;

#[component]
pub fn UserProfile(
    #[prop(optional)] show_email: bool,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let auth_state = use_auth_state();
    let current_user = use_current_user();
    
    let container_classes = format!(
        "flex items-center space-x-3 {}",
        class.unwrap_or("")
    );

    view! {
        <div class=container_classes>
            {move || match auth_state.get() {
                AuthState::Loading => view! {
                    <div class="flex items-center space-x-2">
                        <div class="animate-spin rounded-full h-6 w-6 border-2 border-gray-300 border-t-blue-600"></div>
                        <span class="text-sm text-gray-500">"Loading..."</span>
                    </div>
                }.into_any(),
                
                AuthState::Authenticated(_) => {
                    let user = current_user.get().unwrap();
                    view! {
                        <div class="flex items-center space-x-3">
                            // User avatar
                            {user.picture.as_ref().map(|pic| view! {
                                <img 
                                    src=pic.clone()
                                    alt="Profile"
                                    class="w-8 h-8 rounded-full border border-gray-200"
                                />
                            })}
                            
                            // User info
                            <div class="flex flex-col">
                                <span class="text-sm font-medium text-gray-900">
                                    {user.name.as_ref().unwrap_or(&user.email).clone()}
                                </span>
                                {show_email.then(|| view! {
                                    <span class="text-xs text-gray-500">{user.email.clone()}</span>
                                })}
                            </div>
                            
                            // Logout button
                            <LogoutButton />
                        </div>
                    }.into_any()
                },
                
                AuthState::Unauthenticated => view! {
                    <LoginButton />
                }.into_any(),
            }}
        </div>
    }
}

#[component]
pub fn UserAvatar(
    #[prop(optional)] size: Option<u32>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let current_user = use_current_user();
    let size = size.unwrap_or(32);
    let avatar_classes = format!(
        "rounded-full border border-gray-200 {}",
        class.unwrap_or("")
    );

    view! {
        {move || match current_user.get() {
            Some(user) => {
                if let Some(picture) = &user.picture {
                    view! {
                        <img 
                            src=picture.clone()
                            alt="Profile"
                            class=avatar_classes.clone()
                            width=size
                            height=size
                        />
                    }.into_any()
                } else {
                    // Default avatar with initials
                    let initials = user.name
                        .as_ref()
                        .or(Some(&user.email))
                        .map(|name| {
                            name.split_whitespace()
                                .take(2)
                                .map(|word| word.chars().next().unwrap_or('?').to_uppercase().to_string())
                                .collect::<Vec<_>>()
                                .join("")
                        })
                        .unwrap_or_else(|| "?".to_string());
                    
                    view! {
                        <div 
                            class=format!("flex items-center justify-center bg-blue-500 text-white text-sm font-medium {}", avatar_classes)
                            style=format!("width: {}px; height: {}px;", size, size)
                        >
                            {initials}
                        </div>
                    }.into_any()
                }
            },
            None => view! {
                <div 
                    class=format!("flex items-center justify-center bg-gray-300 text-gray-600 text-sm {}", avatar_classes)
                    style=format!("width: {}px; height: {}px;", size, size)
                >
                    "?"
                </div>
            }.into_any(),
        }}
    }
}

#[component]
pub fn UserName(
    #[prop(optional)] fallback_to_email: bool,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let current_user = use_current_user();
    
    let name_classes = format!(
        "text-sm font-medium text-gray-900 {}",
        class.unwrap_or("")
    );

    view! {
        {move || match current_user.get() {
            Some(user) => {
                let display_name = if fallback_to_email {
                    user.name.as_ref().unwrap_or(&user.email).clone()
                } else {
                    user.name.as_ref().unwrap_or(&"Unknown User".to_string()).clone()
                };
                
                view! {
                    <span class=name_classes.clone()>{display_name}</span>
                }.into_any()
            },
            None => view! {
                <span class=name_classes.clone()>"Not signed in"</span>
            }.into_any(),
        }}
    }
}