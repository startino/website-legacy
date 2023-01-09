use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Header(props: &Props) -> Html {
    html! {
        <div class="sticky z-40 top-0 w-full backdrop-blur flex-none transition-colors duration-500 bg-black/5">
            <div class="py-4 border-b border-secondary-300/10 mx-5">
                <div class="px-auto xl:px-40 lg:px-20 md:px-10 md:px-auto relative flex items-center">
                    <a class="px-3 flex overflow-hidden" href="/">
                        <Logo style="labeled" />
                    </a>
                    {for props.children.iter()}

                    <div class="flex items-center ml-auto">
                        <nav class="text-sm leading-6 font-semibold text-primary-400">
                            <ul class="m-auto flex space-x-8">
                                <li>
                                    <TextLink text="Home" href="/" />
                                </li>
                                <li>
                                    <TextLink text="Contact" href="/contact" />
                                </li>
                                <li>
                                    <TextLink text="About Us" href="/about" />
                                </li>
                                <li class="border-l border-secondary-300/10"/>
                                <li class="hover:text-accent-400">
                                    <ThemeButton />
                                </li>
                            </ul>
                        </nav>
                    </div>
                </div>
            </div>
        </div>
    }
}
