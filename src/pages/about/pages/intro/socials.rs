use leptos::prelude::*;
use leptos_icons::Icon;

const EMAIL: &str = "casey.vaughn9@aol.com";

#[component]
pub fn SocialIcons() -> impl IntoView {
    let copy_success = RwSignal::new(false);
    let icon_style = Signal::derive({
        move || {
            if copy_success.get() {
                "transition: color 0.2s ease; color: #22c55e;".to_string()
            } else {
                "transition: color 0.2s ease;".to_string()
            }
        }
    });

    view! {
        <span class="socials flex gap-6 mt-4">
            <a
                href="https://github.com/Flan8er"
                target="_blank"
                rel="noopener noreferrer"
                class="social-button p-2 hover:scale-110 transition origin-center"
            >
                <Icon icon=icondata::ChGithub width="24px" height="24px" style="transition: color 0.2s ease;" />
            </a>

            <a
                href="https://linkedin.com/in/casey-vaughn-1a8ba72b2"
                target="_blank"
                rel="noopener noreferrer"
                class="social-button p-2 hover:scale-110 transition origin-center"
            >
                <Icon icon=icondata::FiLinkedin width="24px" height="24px" style="transition: color 0.2s ease;" />
            </a>

            <a
                href="mailto:info@example.com"
                class="social-button p-2 hover:scale-110 transition origin-center"
            >
                <Icon icon=icondata::LuMail width="24px" height="24px" style=icon_style />
            </a>
        </span>
    }
}
