use dioxus::prelude::*;

use crate::{
    app_model::app_context::AppContext, components::helpers::SortDir, dog_model::breed::Breed,
};

#[derive(Props, PartialEq, Clone)]
pub struct ResetButtonProps {
    pub sort_dir: Signal<SortDir>,
    pub breed_filter: Signal<Option<Breed>>,
}

#[component]
pub fn ResetButton(mut props: ResetButtonProps) -> Element {
    let mut ctx = use_context::<Signal<AppContext>>();

    // Render  ----------------------------------- /
    rsx! {
        button {
            id: "reset-button",
            onclick: move |_| {
                ctx.write().clear_all();
                props.sort_dir.set(SortDir::Asc);
                props.breed_filter.set(None);
            },
            "🔄 Reset"
        }
    }
}
