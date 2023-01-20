use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Footer(props: &Props) -> Html {
    html! {

    <footer class="p-6 rounded-lg shadow border-secondary-300/10">
        <div class="sm:flex sm:justify-between sm:items-center">
                <Logo style="labeled" />
            <ul class="flex flex-wrap items-center mb-6 space-x-6 text-sm text-gray-500 sm:mb-0 dark:text-gray-400">
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
        <hr class="my-6 sm:mx-auto lg:my-8 border-secondary-300/10" />
        <span class="block text-sm text-gray-500 sm:text-center dark:text-gray-400">{"Unpublished © 2022 "}<a href="#" class="hover:underline text-primary-400 hover:text-accent-400">{"Futino™"}</a>
        </span>
    </footer>
        }
}
