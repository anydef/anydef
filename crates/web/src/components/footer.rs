use crate::config::FooterConfig;
use leptos::*;

#[component]
pub fn Footer(config: FooterConfig) -> impl IntoView {
    let links = config.links.clone();
    let links_len = links.len();

    let subtext = config.subtext.clone();

    view! {
        <footer class="footer">
            <p>
                {&config.text}" "
                <For
                    each=move || links.clone().into_iter().enumerate()
                    key=|(_, link)| link.url.clone()
                    children=move |(i, link)| {
                        let is_external = link.url.starts_with("http");
                        view! {
                            <a
                                href={&link.url}
                                target={if is_external { "_blank" } else { "" }}
                                rel={if is_external { "noopener noreferrer" } else { "" }}
                            >
                                {&link.label}
                            </a>
                            {if i < links_len - 2 { ", " } else if i == links_len - 2 { " and " } else { "" }}
                        }
                    }
                />
            </p>
            {subtext.map(|text| view! { <p>{text}</p> })}
        </footer>
    }
}
