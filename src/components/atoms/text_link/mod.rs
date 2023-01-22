use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn TextLink(props: &Props) -> Html {
    let Props { text, href } = props;
    html! {
    <Label>
                            <a class="inline text-secondary-light dark:text-secondary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark"
                                href={href.to_owned()}>
                                {text.to_owned()}
                            </a>
                    </Label>
        }
}
