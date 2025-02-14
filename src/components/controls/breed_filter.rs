use dioxus::prelude::*;

use crate::dog_model::breed::Breed;

#[component]
pub fn BreedFilter(mut selected: Signal<Option<Breed>>) -> Element {
    let name = "breed_filter";
    rsx! {
        div {
            id: "breedfilter",
            form {
                fieldset {
                    legend { "Filter by breed" }
                    select { name, id: name,
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
