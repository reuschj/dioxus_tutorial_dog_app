use dioxus::prelude::*;

use crate::dog_model::breed::Breed;

fn get_value(selected: &Signal<Option<Breed>>) -> String {
    match &*selected.read() {
        Some(breed) => breed.get_value(),
        None => String::from("none"),
    }
}

#[component]
pub fn BreedFilter(mut selected: Signal<Option<Breed>>) -> Element {
    let name = "breed_filter";
    rsx! {
        div {
            id: "breedfilter",
            form {
                fieldset {
                    legend { "Show breed" }
                    select { name, id: name, value: get_value(&selected),
                        onchange: move |e| {
                            selected.set(Breed::from_raw_str(e.value()));
                        },
                        option { value: "none", "All" }
                        option { value: "corgi", "Corgi" }
                        option { value: "dalmation", "Dalmation" }
                        option { value: "lab", "Lab" }
                        option { value: "husky", "Husky" }
                        option { value: "poodle", "Poodle" }
                        option { value: "retriever", "Retriever" }
                        option { value: "sheperd", "Sheperd" }
                    }
                }
            }
        }
    }
}
