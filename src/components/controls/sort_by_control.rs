use dioxus::prelude::*;

use crate::components::helpers::SortDir;

#[component]
pub fn SortByControl(mut selected: Signal<SortDir>) -> Element {
    let name = "sort_by";
    let input_type = "radio";
    let asc = "asc";
    let dsc = "dsc";

    rsx! {
        div {
            id: "sortbycontrol",
            form {
                fieldset {
                    legend { "Sort by" }

                    div {
                        input { type: input_type, id: asc, name, value: asc, checked: "true", onclick: move |_| {
                            selected.set(SortDir::Asc);
                        } }
                        label { for: asc, "Ascending" }
                    }

                    div {
                        input { type: input_type, id: dsc, name, value: dsc, onclick: move |_| {
                            selected.set(SortDir::Dsc);
                        } }
                        label { for: dsc, "Descending" }
                    }
                }
            }
        }
    }
}
