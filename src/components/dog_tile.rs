use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    app_model::app_context::AppContext,
    components::controls::button::{Button, ButtonMode, ButtonType},
};

#[derive(PartialEq, Clone, Copy)]
pub enum DogTileViewMode {
    Normal,
    Mini,
}

#[derive(Props, PartialEq, Clone)]
pub struct DogTileProps {
    pub dog_id: Uuid,
    pub view_mode: DogTileViewMode,
}

#[component]
pub fn DogTile(props: DogTileProps) -> Element {
    // Context --------------------------------------- /
    let ctx = use_context::<Signal<AppContext>>();

    // Dog ------------------------------------------- /
    let dog = match ctx.read().db.lock() {
        Result::Ok(db) => match db.get(props.dog_id) {
            Result::Ok(dog) => Some(dog),
            Err(_) => None,
        },
        Err(_) => None,
    };
    if dog.is_none() {
        return rsx!();
    }
    let dog = dog.unwrap();
    let is_favorite = ctx.read().favorites.read().contains(&props.dog_id);

    // Render ------------------------------------------- /
    match props.view_mode {
        // Normal tile style --------------- /
        DogTileViewMode::Normal => {
            rsx! {
                div {
                    class: "dogtile",
                    class: if is_favorite {
                        "favorite"
                    },
                    div { class: "header", h2 { class: "title", "{dog.name}" }, h3 { class: "subtitle", "{dog.breed}" } }
                    if let Some(description) = dog.description {
                        p { class: "description", "{description}" }
                    }
                    if let Some(src) = dog.image {
                        img { src: "{src}", class: "dogimage", max_height: "300px" }
                    }
                    div { class: "buttons",
                        if !is_favorite {
                            Button { button_type: ButtonType::Archive, dog_id: dog.id, mode: ButtonMode::Apply }
                        }
                        match is_favorite {
                            true => rsx! { Button { button_type: ButtonType::Favorite, dog_id: dog.id, mode: ButtonMode::Remove } },
                            false => rsx! { Button { button_type: ButtonType::Favorite, dog_id: dog.id, mode: ButtonMode::Apply } }
                        }
                    }
                }
            }
        }
        // Mini tile style --------------- /
        DogTileViewMode::Mini => rsx! {
            div { class: "dogtile", class: "mini",
                div { class: "header", h2 { class: "title", "{dog.name}" }, h3 { class: "subtitle", "{dog.breed}" } }
                if let Some(src) = dog.image {
                    img { src: "{src}", class: "dogimage", max_height: "300px" }
                }
                div { class: "buttons",
                    Button { dog_id: dog.id, button_type: ButtonType::Archive, mode: ButtonMode::Remove }
                }
            }
        },
    }
}
