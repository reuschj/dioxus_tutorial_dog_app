use dioxus::{document::document, prelude::*};

use crate::{
    app_model::{app_context::AppContext, theme::Theme},
    components::{
        controls::{breed_filter::BreedFilter, sort_by_control::SortByControl},
        dogs_view::DogsView,
        helpers::SortDir,
    },
    constants::{APP_NAME, CSS, GOOGLE_FONT_API, GOOGLE_FONT_STATIC, GOOGLE_FONT_URL},
    dog_model::{breed::Breed, dog_db::DogDb},
};

#[component]
pub fn App() -> Element {
    // Init the dog database from JSON file ----------------------- /
    let dog_data = include_str!("../assets/dog_data.json");
    let dog_db = DogDb::from_json(dog_data).unwrap_or_else(|e| {
        log::error!("{}", e);
        DogDb::new()
    });

    // State ------------------------------------------------------ /
    let sort_dir = use_signal(|| SortDir::Asc);
    let breed_filter = use_signal::<Option<Breed>>(|| None);

    // Context ------------------------------------------------------ /
    use_context_provider(|| Signal::new(AppContext::new(dog_db)));
    use_context_provider(|| Signal::new(Theme::System));

    // theme ------------------------------------------------------ /
    let theme_name = use_context::<Signal<Theme>>().read().class_name();

    // use_effect(|| return document().body.classname("{theme_name}"));

    // Render ------------------------------------------------------ /
    rsx! {
        document::Link { href: GOOGLE_FONT_API }
        document::Link { href: GOOGLE_FONT_STATIC, crossorigin: "true" }
        document::Link { href: GOOGLE_FONT_URL, rel: "stylesheet" }
        document::Stylesheet { href: CSS }
        div {
            class: "app",
            class: "{theme_name}",
            div {
                h1 { id: "maintitle", "{APP_NAME}" }
                span { id: "maindesc", "An app for sorting dog photos." }
            }
            div {
                id: "controlbox",
                SortByControl { selected: sort_dir }
                BreedFilter { selected: breed_filter }
            }
            DogsView { sort_dir, breed_filter }
        }
    }
}
