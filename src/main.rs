mod components;
mod constants;
mod enums;
mod layout;
mod pages;

// // import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{
    hooks::use_context_provider,
    prelude::{component, dioxus_core, fc_to_builder, launch, rsx, Element, Router}, signals::Signal,
};
use tracing::Level;

// use dioxus_router::prelude::Router;

fn main() {
    //     // launch the web app
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);

    //     dioxus_web::launch(App);
    //     wasm_logger::init(wasm_logger::Config::default());
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new("zh-TW"));
    rsx! {
        Router::<enums::route::Route> {}
    }
}
