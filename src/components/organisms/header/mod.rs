use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Header(props: &Props) -> Html {
    html! {
        <div class="sticky z-40 top-0 w-full backdrop-blur flex-none transition-colors duration-500 bg-black/5">
            <div class="py-4 border-b border-pink-300/10 mx-5">
                <div class="px-auto xl:px-40 lg:px-20 md:px-10 md:px-auto relative flex items-center">
                    <a class="px-3 flex overflow-hidden" href="#">
                        <Logo style="labeled" />
                    </a>
                    {for props.children.iter()}

                    <div class="flex items-center ml-auto">
                        <nav class="text-sm leading-6 font-semibold text-purple-400">
                            <ul class="m-auto flex space-x-8">
                                <li class="hover:text-emerald-400">
                                    <a href="/">{"Home"}</a>
                                </li>
                                <li class="hover:text-emerald-400">
                                    <a href="/contact">{"Contact"}</a>
                                </li>
                                <li class="hover:text-emerald-400">
                                    <a href="/about">{"About Us"}</a>
                                </li>
                                <li class="border-l border-pink-300/10"/>
                                <li class="hover:text-emerald-400">
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