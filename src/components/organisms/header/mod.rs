use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Header(props: &Props) -> Html {
    html! {
        <div class="sticky top-0 z-40 flex-none w-full transition-colors duration-500 backdrop-blur bg-black/5">
            <div class="py-4 mx-5 border-b border-secondary-300/10">
                <div class="flex relative items-center md:px-10 lg:px-20 xl:px-40 px-auto md:px-auto">
                    <a class="flex overflow-hidden px-3" href="/">
                        <Logo style="labeled" />
                    </a>
                    {for props.children.iter()}

                    <div class="flex items-center ml-auto">
                        <nav class="text-sm font-semibold leading-6 text-primary">
                            <ul class="flex m-auto space-x-8">
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
