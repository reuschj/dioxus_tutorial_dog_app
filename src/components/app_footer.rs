use dioxus::prelude::*;

use crate::{components::controls::reset_button::ResetButton, dog_model::breed::Breed};

use super::helpers::SortDir;

#[derive(Props, PartialEq, Clone)]
pub struct AppFooterProps {
    pub sort_dir: Signal<SortDir>,
    pub breed_filter: Signal<Option<Breed>>,
}

#[component]
pub fn AppFooter(props: AppFooterProps) -> Element {
    rsx! {
        div {
            class: "app-footer",
            div {
                class: "app-footer-actions",
                div {
                    class: "app-footer-action",
                    ResetButton {
                        sort_dir: props.sort_dir,
                        breed_filter: props.breed_filter,
                    }
                }
            }
            div {
                class: "app-footer-content",
                div {
                    "Copyright © 2023 Dioxus Tutorial Dog App"
                }
            }
        }
    }
}
