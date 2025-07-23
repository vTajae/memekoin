use leptos::prelude::*;
use leptos::ev;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn Button(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(ButtonVariant::Primary);
    let size = size.unwrap_or(ButtonSize::Medium);
    let disabled = disabled.unwrap_or(false);
    let additional_class = class.unwrap_or("");

    let variant_classes = match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary", 
        ButtonVariant::Outline => "btn-outline",
        ButtonVariant::Ghost => "btn-ghost",
        ButtonVariant::Destructive => "bg-red-600 text-white hover:bg-red-700 active:bg-red-800",
    };

    let size_classes = match size {
        ButtonSize::Small => "btn-sm",
        ButtonSize::Medium => "btn-md",
        ButtonSize::Large => "btn-lg",
    };

    let classes = format!(
        "btn {} {} {}",
        variant_classes,
        size_classes,
        additional_class
    );

    view! {
        <button
            class=classes
            disabled=disabled
            on:click=move |ev| {
                if let Some(handler) = on_click {
                    handler.run(ev);
                }
            }
        >
            {children()}
        </button>
    }
}
