use rust_hello_dioxus::app;

fn main() {
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}
