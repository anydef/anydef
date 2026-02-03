use leptos::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Cv {
    personal: Personal,
    experience: Vec<Experience>,
    education: Vec<Education>,
    skills: Skills,
    interests: Vec<Interest>,
}

#[derive(Deserialize)]
struct Personal {
    name: String,
    title: String,
    location: String,
    intro: String,
    email: String,
    github: String,
    linkedin: String,
}

#[derive(Deserialize, Clone)]
struct Experience {
    title: String,
    company: String,
    date: String,
    skills: Vec<String>,
    description: String,
}

#[derive(Deserialize, Clone)]
struct Education {
    degree: String,
    institution: String,
    date: String,
}

#[derive(Deserialize, Clone)]
struct Skills {
    languages: Vec<String>,
    frameworks: Vec<String>,
    infrastructure: Vec<String>,
    observability: Vec<String>,
    data: Vec<String>,
}

#[derive(Deserialize, Clone)]
struct Interest {
    name: String,
    description: String,
}

fn load_cv() -> Cv {
    const CV_YAML: &str = include_str!("../../../../cv.yaml");
    serde_yaml::from_str(CV_YAML).expect("Failed to parse cv.yaml")
}

#[component]
pub fn Bio() -> impl IntoView {
    let cv = load_cv();

    view! {
        <section class="bio">
            <div class="bio-content">
                <div class="bio-header">
                    <h2>"About Me"</h2>
                </div>

                <div class="bio-section">
                    <p class="bio-intro">{&cv.personal.intro}</p>
                </div>

                <div class="bio-section">
                    <h3>"Experience"</h3>
                    <For
                        each=move || cv.experience.clone()
                        key=|exp| format!("{}-{}", exp.company, exp.date)
                        children=move |exp| {
                            let skills = exp.skills.clone();
                            view! {
                                <div class="bio-item">
                                    <div class="bio-item-header">
                                        <span class="bio-item-title">{&exp.title}</span>
                                        <span class="bio-item-date">{&exp.date}</span>
                                    </div>
                                    <span class="bio-item-company">{&exp.company}</span>
                                    <p>{&exp.description}</p>
                                    <div class="bio-item-skills">
                                        <For
                                            each=move || skills.clone()
                                            key=|skill| skill.clone()
                                            children=move |skill| view! {
                                                <span class="bio-item-skill">{skill}</span>
                                            }
                                        />
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>

                <div class="bio-section">
                    <h3>"Skills"</h3>

                    <div class="bio-skills-category">
                        <h4>"Languages"</h4>
                        <div class="bio-skills">
                            <For
                                each=move || cv.skills.languages.clone()
                                key=|skill| skill.clone()
                                children=move |skill| view! {
                                    <span class="bio-skill">{skill}</span>
                                }
                            />
                        </div>
                    </div>

                    <div class="bio-skills-category">
                        <h4>"Frameworks"</h4>
                        <div class="bio-skills">
                            <For
                                each=move || cv.skills.frameworks.clone()
                                key=|skill| skill.clone()
                                children=move |skill| view! {
                                    <span class="bio-skill">{skill}</span>
                                }
                            />
                        </div>
                    </div>

                    <div class="bio-skills-category">
                        <h4>"Infrastructure"</h4>
                        <div class="bio-skills">
                            <For
                                each=move || cv.skills.infrastructure.clone()
                                key=|skill| skill.clone()
                                children=move |skill| view! {
                                    <span class="bio-skill">{skill}</span>
                                }
                            />
                        </div>
                    </div>

                    <div class="bio-skills-category">
                        <h4>"Observability"</h4>
                        <div class="bio-skills">
                            <For
                                each=move || cv.skills.observability.clone()
                                key=|skill| skill.clone()
                                children=move |skill| view! {
                                    <span class="bio-skill">{skill}</span>
                                }
                            />
                        </div>
                    </div>

                    <div class="bio-skills-category">
                        <h4>"Data"</h4>
                        <div class="bio-skills">
                            <For
                                each=move || cv.skills.data.clone()
                                key=|skill| skill.clone()
                                children=move |skill| view! {
                                    <span class="bio-skill">{skill}</span>
                                }
                            />
                        </div>
                    </div>
                </div>

                <div class="bio-section">
                    <h3>"Education"</h3>
                    <For
                        each=move || cv.education.clone()
                        key=|edu| edu.degree.clone()
                        children=move |edu| view! {
                            <div class="bio-item">
                                <div class="bio-item-header">
                                    <span class="bio-item-title">{&edu.degree}</span>
                                    <span class="bio-item-date">{&edu.date}</span>
                                </div>
                                <span class="bio-item-company">{&edu.institution}</span>
                            </div>
                        }
                    />
                </div>

                <div class="bio-section">
                    <h3>"Interests"</h3>
                    <div class="bio-interests">
                        <For
                            each=move || cv.interests.clone()
                            key=|interest| interest.name.clone()
                            children=move |interest| view! {
                                <div class="bio-interest">
                                    <span class="bio-interest-name">{&interest.name}</span>
                                    <p class="bio-interest-desc">{&interest.description}</p>
                                </div>
                            }
                        />
                    </div>
                </div>

                <div class="bio-section">
                    <h3>"Contact"</h3>
                    <div class="bio-contact">
                        <span class="bio-location">{&cv.personal.location}</span>
                        <a href={format!("mailto:{}", cv.personal.email)}>{&cv.personal.email}</a>
                        <a href={cv.personal.github.clone()} target="_blank" rel="noopener noreferrer">"GitHub"</a>
                        <a href={cv.personal.linkedin.clone()} target="_blank" rel="noopener noreferrer">"LinkedIn"</a>
                    </div>
                </div>
            </div>
        </section>
    }
}
