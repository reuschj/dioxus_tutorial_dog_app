// Modules ------------------------------------------- /
mod app_model {
    pub mod app_context;
    pub mod theme;
}

mod components {
    pub mod controls {
        pub mod breed_filter;
        pub mod button;
        pub mod reset_button;
        pub mod sort_by_control;
    }
    pub mod app_footer;
    pub mod app_header;
    pub mod dog_tile;
    pub mod dogs_view;
    pub mod helpers;
}

mod dog_model {
    pub mod breed;
    pub mod dog;
    pub mod dog_db;
}

mod util {
    pub mod app_local_storage;
}

mod app;
mod constants;

// Main ------------------------------------------- /

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::launch(app::App);
}
