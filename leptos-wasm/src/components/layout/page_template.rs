use leptos::prelude::*;
use crate::components::layout::header::Header;
use crate::components::layout::footer::Footer;

/// Page template component that provides consistent layout with header and footer
#[component]
pub fn PageTemplate(
    /// Optional page title to display
    #[prop(optional)]
    title: Option<String>,
    /// Optional additional CSS classes for the main content area
    #[prop(optional)]
    class: Option<String>,
    /// The page content
    children: Children,
) -> impl IntoView {
    let main_class = move || {
        format!(
            "flex-1 {} {}",
            "w-full",
            class.as_ref().unwrap_or(&String::new())
        )
    };

    view! {
        <div class="min-h-screen flex flex-col bg-gray-50">
            <Header />
            
            <main class=main_class>
                {title.map(|t| view! {
                    <div class="bg-white shadow-sm border-b border-gray-200">
                        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
                            <h1 class="text-2xl font-bold text-gray-900">{t}</h1>
                        </div>
                    </div>
                })}
                
                <div class="flex-1">
                    {children()}
                </div>
            </main>
            
            <Footer />
        </div>
    }
}