use std::sync::Arc;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    app_model::app_context::AppContext,
    components::dog_tile::{DogTile, DogTileViewMode},
    dog_model::{breed::Breed, dog::Dog},
};

use super::helpers::SortDir;

#[derive(Props, PartialEq, Clone)]
pub struct DogsViewProps {
    pub sort_dir: Signal<SortDir>,
    pub breed_filter: Signal<Option<Breed>>,
}

#[component]
pub fn DogsView(props: DogsViewProps) -> Element {
    let sort_dir = props.sort_dir.read();
    let breed_filter = props.breed_filter.read();

    let ctx = use_context::<Signal<AppContext>>();

    let db = Arc::clone(&ctx.read().db);
    let dog_db = db.as_ref().lock().unwrap();
    let archived = ctx.read().archived;
    let favorites = ctx.read().favorites;

    // Prepare all dogs from db ----------------------------------- /
    // Filter dogs ---- /
    let mut all_dogs: Vec<Dog> = dog_db
        .iter_all()
        .filter(|dog| match breed_filter.as_ref() {
            Some(selected_breed) => dog.breed == *selected_breed,
            None => true,
        })
        .collect();

    // Sort dogs ---- /
    all_dogs.sort_by(|a, b| {
        let a_is_fav = favorites.read().contains(&a.id);
        let b_is_fav = favorites.read().contains(&b.id);
        if a_is_fav && !b_is_fav {
            std::cmp::Ordering::Less
        } else if !a_is_fav && b_is_fav {
            std::cmp::Ordering::Greater
        } else {
            match *sort_dir {
                SortDir::Asc => a.name.cmp(&b.name),
                SortDir::Dsc => b.name.cmp(&a.name),
            }
        }
    });
    // Partition dogs ids
    let (archived_dogs, dogs_to_review): (Vec<Uuid>, Vec<Uuid>) = all_dogs
        .iter()
        .map(|dog| dog.id)
        .partition(|id| archived.read().contains(id));

    // Render ----------------------------------- /
    rsx! {
        div {
            if dogs_to_review.len() > 0 {
                div {
                    class: "dogsview",
                    for dog_id in dogs_to_review {
                        DogTile {
                            dog_id,
                            view_mode: DogTileViewMode::Normal
                        }
                    }
                }
            } else {
                div { "No dogs to review!" }
            }
            if archived_dogs.len() > 0 {
                div {
                    class: "archived",
                    h4 {
                        class: "title",
                        "Archived"
                    }
                    div {
                        class: "dogsview",
                        for dog_id in archived_dogs {
                            DogTile {
                                dog_id,
                                view_mode: DogTileViewMode::Mini
                            }
                        }
                    }
                }
            }
        }
    }
}
