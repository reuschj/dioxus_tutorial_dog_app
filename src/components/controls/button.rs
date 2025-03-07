use dioxus::prelude::*;
use uuid::Uuid;

use crate::app_model::app_context::AppContext;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonType {
    Archive,
    Favorite,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonMode {
    Apply,
    Remove,
}

#[derive(Props, Clone, Copy, PartialEq)]
pub struct ButtonProps {
    pub dog_id: Uuid,
    pub button_type: ButtonType,
    pub mode: ButtonMode,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let id = props.dog_id;

    // Context  ----------------------------------- /
    let ctx = use_context::<Signal<AppContext>>();
    let mut id_set = match props.button_type {
        ButtonType::Archive => ctx.read().archived,
        ButtonType::Favorite => ctx.read().favorites,
    };
    // Get dog ---- /
    let dog = match ctx.read().db.lock() {
        Result::Ok(db) => match db.get(id) {
            Result::Ok(dog) => Some(dog),
            Err(_) => None,
        },
        Err(_) => None,
    };

    // Render  ----------------------------------- /
    rsx! {
        button {
            id: match props.button_type {
                ButtonType::Archive => match props.mode {
                    ButtonMode::Apply => "archive",
                    ButtonMode::Remove => "restore"
                },
                ButtonType::Favorite => match props.mode {
                    ButtonMode::Apply => "favorite",
                    ButtonMode::Remove => "unfavorite"
                },
            },
            onclick: move |_| {
                if let Some(dog) = &dog {
                    let log_desc = match props.button_type {
                        ButtonType::Archive => match props.mode {
                            ButtonMode::Apply => "Archived",
                            ButtonMode::Remove => "Restored"
                        },
                        ButtonType::Favorite => match props.mode {
                            ButtonMode::Apply => "Favorited",
                            ButtonMode::Remove => "Unfavorited"
                        },
                    };
                    log::debug!("{} {}! ({})", log_desc, dog.name, dog.breed);
                }
                match props.mode {
                    ButtonMode::Apply => id_set.write().insert(id),
                    ButtonMode::Remove => id_set.write().remove(&id),
                };
                match props.button_type {
                    ButtonType::Archive => ctx.read().save_archived(),
                    ButtonType::Favorite => ctx.read().save_favorites(),
                };

            },
            match props.button_type {
                ButtonType::Archive => match props.mode {
                    ButtonMode::Apply => "🗑️ Archive",
                    ButtonMode::Remove => "⏫ Restore"
                },
                ButtonType::Favorite => match props.mode {
                    ButtonMode::Apply => "⭐️ Favorite",
                    ButtonMode::Remove => "🔽 Unfavorite"
                },
            }
        }
    }
}
