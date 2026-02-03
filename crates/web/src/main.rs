mod components;
mod config;

use components::{Bio, Footer, Header, Social};
use config::load_config;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let config = load_config();
    let (show_bio, set_show_bio) = create_signal(false);
    let (show_social, set_show_social) = create_signal(false);

    let hero = config.hero.clone();
    let bio_label = hero.buttons.iter().find(|b| b.style == "primary").map(|b| b.label.clone()).unwrap_or_default();
    let social_label = hero.buttons.iter().find(|b| b.style == "secondary").map(|b| b.label.clone()).unwrap_or_default();
    // let cv_button = hero.buttons.iter().find(|b| b.style == "link").cloned();

    view! {
        <>
            <Header config=config.header.clone()/>

            <section class="hero">
                <h1>{&hero.title}</h1>
                <p>{&hero.subtitle}</p>
                <div class="cta-buttons">
                    <button
                        class=move || if show_bio.get() { "btn btn-primary active" } else { "btn btn-primary" }
                        on:click=move |_| {
                            set_show_bio.update(|v| *v = !*v);
                            set_show_social.set(false);
                        }
                    >
                        {&bio_label}
                    </button>
                    <button
                        class=move || if show_social.get() { "btn btn-secondary active" } else { "btn btn-secondary" }
                        on:click=move |_| {
                            set_show_social.update(|v| *v = !*v);
                            set_show_bio.set(false);
                        }
                    >
                        {&social_label}
                    </button>
                    // {cv_button.map(|btn| view! {
                    //     <a
                    //         class="btn btn-link"
                    //         href={btn.url.unwrap_or_default()}
                    //         download
                    //     >
                    //         {btn.label}
                    //     </a>
                    // })}
                </div>
            </section>

            <Show when=move || show_bio.get()>
                <Bio/>
            </Show>

            <Show when=move || show_social.get()>
                <Social/>
            </Show>

            <Footer config=config.footer.clone()/>
        </>
    }
}
