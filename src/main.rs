use yew::prelude::*;

pub mod components;

use components::topbar::Topbar;
use components::button::Button;
use components::theme_button::ThemeButton;

#[function_component(App)]
fn app() -> Html {

    // many <br/>, get rid of it when page actually has content
    let brs = (0..999).collect::<Vec<i32>>().iter().map(|_| html! {<br/>}).collect::<Html>();

    let logo = html! {<img src="images/logo/circle/1024.png" class="h-9 rounded-full" alt="Logo"/>};
    html! {
        <div class="bg-white dark:bg-black">
        <div class="pb-20 bg-gradient-to-r from-purple-900/30 to-teal-900/30 antialiased">
            <Topbar>
                <a class="px-3 flex overflow-hidden" href="#">
                    <img src="images/logo/circle/1024.png" class="h-9 rounded-full" alt="Logo"/>
                </a>

                <div class="flex items-center ml-auto">
                    <nav class="text-sm leading-6 font-semibold text-purple-400">
                        <ul class="m-auto flex space-x-8">
                            <li class="hover:text-emerald-400">
                                <a href="#">{"Home"}</a>
                            </li>
                            <li class="hover:text-emerald-400">
                                <a href="#">{"Contact"}</a>
                            </li>
                            <li class="border-l border-pink-300/10"/>
                            <li class="hover:text-emerald-400">
                                <ThemeButton/>
                            </li>
                        </ul>
                    </nav>
                </div>
            </Topbar>
            <div class="py-40 px-4 sm:px-6 md:px-8 border-b border-pink-500/40 shadow-2xl">
                <div class="relative max-w-5xl mx-auto">
                    <h1 class="font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center text-white">
                        {"Futino"}
                    </h1>

                    <p class="mt-6 text-lg text-white text-center max-w-3xl mx-auto">
                        {"Dynamically built web-apps with "}
                        <a class="inline text-purple-400 hover:text-emerald-400"
                            href="https://en.wikipedia.org/wiki/Free_and_open-source_software">
                            {"Free and Open-Source Software!"}
                        </a>
                    </p>

                    <div class="mt-6 sm:mt-10 flex justify-center space-x-6 text-sm">
                        <Button text={"Contact"} href={"#"}/>
                        <Button text={"Why choose us?"} href={"#"}/>
                    </div>
                </div>
            </div>

            // temp, get rid of when page has content
            {brs}

        </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}