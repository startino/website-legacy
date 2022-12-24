use yew::{Properties, Html};
use yew::prelude::*;

#[function_component]
pub fn Topbar() -> Html {
    html! {
        <div class="sticky z-40 top-0 w-full backdrop-blur flex-none transition-colors duration-500 bg-black/5">
            <div class="py-4 border-b border-pink-300/10 mx-5">
                <div class="px-auto xl:px-40 lg:px-20 md:px-10 md:px-auto relative flex items-center">
                    <a class="px-3 flex overflow-hidden" href="#">
                        <img src="images/logo/circle/1024.png" class="h-9 rounded-full" alt="Logo" />
                    </a>

                    <div class="relative items-center ml-auto">
                        <nav class="text-sm leading-6 font-semibold text-purple-400 hover:text-emerald-400">
                            <ul class="flex space-x-8">
                                <li>
                                    <a href="#">{"Nav Item"}</a>
                                </li>
                            </ul>
                        </nav>
                    </div>
                </div>
            </div>
        </div>
    }
}