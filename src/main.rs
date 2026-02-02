use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <main class="container">
            <h1>"Hello, World!"</h1>
            <p>"Welcome to anydef's page"</p>
            <div class="counter">
                <button on:click=move |_| set_count.update(|n| *n -= 1)>"-"</button>
                <span class="count">{count}</span>
                <button on:click=move |_| set_count.update(|n| *n += 1)>"+"</button>
            </div>
            <p class="hint">"A simple Leptos app running on WebAssembly"</p>
        </main>
    }
}
