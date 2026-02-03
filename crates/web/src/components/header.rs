use crate::config::HeaderConfig;
use leptos::*;

#[component]
pub fn Header(config: HeaderConfig) -> impl IntoView {
    view! {
        <header class="header">
            <div class="header-content">
                <a href="/" class="logo">{&config.logo}</a>
                <nav class="nav">
                    <For
                        each=move || config.nav.clone()
                        key=|link| link.url.clone()
                        children=move |link| {
                            let is_external = link.url.starts_with("http");
                            view! {
                                <a
                                    href={&link.url}
                                    target={if is_external { "_blank" } else { "" }}
                                    rel={if is_external { "noopener noreferrer" } else { "" }}
                                >
                                    {&link.label}
                                </a>
                            }
                        }
                    />
                </nav>
            </div>
        </header>
    }
}
