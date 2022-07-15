use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    let fut = use_future(&cx, (), |_| async move {
        reqwest::get("https://c3r5v-eaaaa-aaaah-qc3uq-cai.raw.ic0.app/v1/api/greet")
            .await
            .unwrap()
            .text()
            .await
    });
    cx.render(match fut.value() {
        Some(Ok(resp)) => rsx! {
            div {
                "hello wasm {resp}"
            }
        },
        Some(Err(_)) => rsx! { div { "loading dogs failed" } },
        None => rsx! { div { "loading dogs..." } },
    })
}
