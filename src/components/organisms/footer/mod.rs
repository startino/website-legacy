use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Footer(props: &Props) -> Html {
    html! {

    <footer class="p-6 rounded-lg shadow border-primary-light/40 dark:border-primary-dark/40">
        <div class="sm:flex sm:justify-between sm:items-center">
                <Logo style="labeled" />
            <ul class="flex flex-wrap items-center mb-6 space-x-6 text-sm  sm:mb-0 text-background-on-light dark:text-background-on-dark">
                <li>
                    <TextLink text="About" href="/about" />
                </li>
                <li>
                    <TextLink text="Privacy Policy" href="#" />
                </li>
                <li>
                    <TextLink text="Licensing" href="#" />
                </li>
                <li>
                    <TextLink text="Contact" href="/contact" />
                </li>
            </ul>
        </div>
        <hr class="my-6 sm:mx-auto lg:my-8 border-primary-light/40 dark:border-primary-dark/40" />
        <span class="block text-sm sm:text-center text-background-on-light dark:text-background-on-dark">{"Unpublished © 2023 "}<a href="#" class="hover:underline text-primary-light hover:text-tertiary-light dark:text-primary-light dark:hover:text-tertiary-light">{"Futino™"}</a>
        </span>
    </footer>
        }
}
