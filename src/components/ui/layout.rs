use leptos::prelude::*;

#[component]
pub fn Container(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let additional_class = class.unwrap_or("");
    let classes = format!("container {}", additional_class);

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

#[component]
pub fn Grid(
    #[prop(optional)] cols: Option<u8>,
    #[prop(optional)] gap: Option<u8>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let cols = cols.unwrap_or(1);
    let gap = gap.unwrap_or(4);
    let additional_class = class.unwrap_or("");
    
    let grid_cols = match cols {
        1 => "grid-cols-1",
        2 => "grid-cols-1 md:grid-cols-2",
        3 => "grid-cols-1 md:grid-cols-2 lg:grid-cols-3",
        4 => "grid-cols-1 md:grid-cols-2 lg:grid-cols-4",
        6 => "grid-cols-1 md:grid-cols-3 lg:grid-cols-6",
        _ => "grid-cols-1",
    };
    
    let gap_class = match gap {
        2 => "gap-2",
        4 => "gap-4",
        6 => "gap-6",
        8 => "gap-8",
        _ => "gap-4",
    };
    
    let classes = format!("grid {} {} {}", grid_cols, gap_class, additional_class);

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

#[component]
pub fn Flex(
    #[prop(optional)] direction: Option<&'static str>,
    #[prop(optional)] justify: Option<&'static str>,
    #[prop(optional)] align: Option<&'static str>,
    #[prop(optional)] gap: Option<u8>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let direction = direction.unwrap_or("row");
    let justify = justify.unwrap_or("start");
    let align = align.unwrap_or("start");
    let gap = gap.unwrap_or(4);
    let additional_class = class.unwrap_or("");
    
    let direction_class = match direction {
        "col" => "flex-col",
        "row" => "flex-row",
        _ => "flex-row",
    };
    
    let justify_class = match justify {
        "start" => "justify-start",
        "center" => "justify-center",
        "end" => "justify-end",
        "between" => "justify-between",
        "around" => "justify-around",
        "evenly" => "justify-evenly",
        _ => "justify-start",
    };
    
    let align_class = match align {
        "start" => "items-start",
        "center" => "items-center",
        "end" => "items-end",
        "stretch" => "items-stretch",
        "baseline" => "items-baseline",
        _ => "items-start",
    };
    
    let gap_class = match gap {
        2 => "gap-2",
        4 => "gap-4",
        6 => "gap-6",
        8 => "gap-8",
        _ => "gap-4",
    };
    
    let classes = format!(
        "flex {} {} {} {} {}",
        direction_class, justify_class, align_class, gap_class, additional_class
    );

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}
