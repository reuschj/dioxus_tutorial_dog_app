use dioxus::prelude::*;

use crate::components::helpers::SortDir;

fn is_selected(selected: &Signal<SortDir>, value: SortDir) -> &str {
    let is_selected = &*selected.read() == &value;
    match is_selected {
        true => "true",
        false => "false",
    }
}

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
                    legend { "Sort" }

                    div {
                        input { type: input_type, id: asc, name, value: asc, checked: is_selected(&selected, SortDir::Asc), onclick: move |_| {
                            selected.set(SortDir::Asc);
                        } }
                        label { for: asc, "Ascending" }
                    }

                    div {
                        input { type: input_type, id: dsc, name, value: dsc, checked: is_selected(&selected, SortDir::Dsc), onclick: move |_| {
                            selected.set(SortDir::Dsc);
                        } }
                        label { for: dsc, "Descending" }
                    }
                }
            }
        }
    }
}
