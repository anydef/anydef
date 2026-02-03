use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub header: HeaderConfig,
    pub hero: HeroConfig,
    pub footer: FooterConfig,
}

#[derive(Deserialize, Clone)]
pub struct HeaderConfig {
    pub logo: String,
    pub nav: Vec<NavLink>,
}

#[derive(Deserialize, Clone)]
pub struct NavLink {
    pub label: String,
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct HeroConfig {
    pub title: String,
    pub subtitle: String,
    pub buttons: Vec<ButtonConfig>,
}

#[derive(Deserialize, Clone)]
pub struct ButtonConfig {
    pub label: String,
    pub style: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct FooterConfig {
    pub text: String,
    pub links: Vec<NavLink>,
    pub subtext: Option<String>,
}

pub fn load_config() -> Config {
    const CONFIG_YAML: &str = include_str!("../../../main.yaml");
    serde_yaml::from_str(CONFIG_YAML).expect("Failed to parse main.yaml")
}
