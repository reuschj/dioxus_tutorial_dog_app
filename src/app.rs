use dioxus::prelude::*;
use gloo_storage::Storage;

use crate::{
    app_model::{app_context::AppContext, theme::Theme},
    components::{
        app_footer::AppFooter, app_header::AppHeader, dogs_view::DogsView, helpers::SortDir,
    },
    constants::{
        BREED_FILTER_KEY, CSS, GOOGLE_FONT_API, GOOGLE_FONT_ICON, GOOGLE_FONT_STATIC,
        GOOGLE_FONT_URL, SORT_DIR_KEY,
    },
    dog_model::{breed::Breed, dog_db::DogDb},
    util::app_local_storage::AppLocalStorage,
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
    let sort_dir = use_signal(|| AppLocalStorage::get(SORT_DIR_KEY).unwrap_or(SortDir::Asc));
    let breed_filter = use_signal::<Option<Breed>>(|| {
        AppLocalStorage::get::<Option<Breed>>(BREED_FILTER_KEY).unwrap_or(None)
    });

    // State subscriptions ----------------------------------------- /

    let sort_dir_clone = sort_dir.clone();
    use_effect(move || {
        let dir = *sort_dir_clone.read();
        AppLocalStorage::set(SORT_DIR_KEY, dir)
            .expect("Failed to set sort direction to local storage.");
        log::info!("Sort direction: {} {}", dir.get_emoji(), dir);
    });

    let breed_filter_clone = breed_filter.clone();
    use_effect(move || {
        let filter = breed_filter_clone.read();
        match &*filter {
            Some(breed) => {
                log::info!("Breed filtered by: {}", breed);
                AppLocalStorage::set(BREED_FILTER_KEY, breed)
                    .expect("Failed to set breed filter to local storage.");
            }
            None => {
                log::info!("No breed filter");
                let _ = AppLocalStorage::remove::<Breed>(BREED_FILTER_KEY);
            }
        }
    });

    // Context ------------------------------------------------------ /
    use_context_provider(|| Signal::new(AppContext::restore_or(dog_db)));
    use_context_provider(|| Signal::new(Theme::System));

    // theme ------------------------------------------------------ /
    let theme_name = use_context::<Signal<Theme>>().read().class_name();

    // use_effect(|| return document().body.classname("{theme_name}")); // TODO: Implement theme change

    // Render ------------------------------------------------------ /
    rsx! {
        document::Link { href: GOOGLE_FONT_API }
        document::Link { href: GOOGLE_FONT_STATIC, crossorigin: "true" }
        document::Link { href: GOOGLE_FONT_URL, rel: "stylesheet" }
        document::Link { href: GOOGLE_FONT_ICON, rel: "stylesheet" }
        document::Stylesheet { href: CSS }
        div {
            class: "app",
            class: "{theme_name}",
            AppHeader { sort_dir, breed_filter }
            DogsView { sort_dir, breed_filter }
            AppFooter { sort_dir, breed_filter }
        }
    }
}
