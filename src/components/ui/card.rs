use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let additional_class = class.unwrap_or("");
    let classes = format!("card {}", additional_class);

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let additional_class = class.unwrap_or("");
    let classes = format!("card-header {}", additional_class);

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

#[component]
pub fn CardContent(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let additional_class = class.unwrap_or("");
    let classes = format!("card-content {}", additional_class);

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let additional_class = class.unwrap_or("");
    let classes = format!("card-footer {}", additional_class);

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}
