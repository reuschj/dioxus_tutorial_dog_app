use dioxus::prelude::*;

use crate::{
    components::controls::{breed_filter::BreedFilter, sort_by_control::SortByControl},
    constants::{APP_DESCRIPTION, APP_NAME},
    dog_model::breed::Breed,
};

use super::helpers::SortDir;

#[derive(Props, PartialEq, Clone)]
pub struct AppHeaderProps {
    pub title: Option<String>,
    pub description: Option<String>,
    pub sort_dir: Signal<SortDir>,
    pub breed_filter: Signal<Option<Breed>>,
}

#[component]
pub fn AppHeader(props: AppHeaderProps) -> Element {
    let title = match props.title {
        Some(title) => title,
        None => APP_NAME.to_string(),
    };
    let description = match props.description {
        Some(description) => description,
        None => APP_DESCRIPTION.to_string(),
    };
    rsx! {
        div {
            class: "header",
            id: "app-header",
            h1 { id: "maintitle", title: "{description}", "{title}" }
            div {
                id: "controlbox",
                SortByControl { selected: props.sort_dir }
                BreedFilter { selected: props.breed_filter }
            }
        }
    }
}
