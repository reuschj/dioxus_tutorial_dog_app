use dioxus::prelude::*;

// App ------------------------------------------------- /

pub static APP_NAME: &str = "Hot Dog!";
pub static APP_DESCRIPTION: &str = "An app for sorting dog photos";

// Keys ------------------------------------------------- /

pub static APP_KEY: &str = "hot_dog_app";

pub static SORT_DIR_KEY: &str = "sort_dir";
pub static BREED_FILTER_KEY: &str = "breed_filter";

// Fonts ------------------------------------------------- /

pub static GOOGLE_FONT_API: &str = "https://fonts.googleapis.com";
pub static GOOGLE_FONT_STATIC: &str = "https://fonts.gstatic.com";
pub static GOOGLE_FONT_URL: &str = "https://fonts.googleapis.com/css2?family=Atma:wght@300;400;500;600;700&family=Roboto:ital,wght@0,100..900;1,100..900&family=Schoolbell&display=swap";

// Icons ------------------------------------------------- /

pub static GOOGLE_FONT_ICON: &str = "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200&icon_names=sort";

// Assets ------------------------------------------------- /
pub static CSS: Asset = asset!("/assets/main.css");
