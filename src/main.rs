use yew::{Properties, Html};
use yew::prelude::*;

#[function_component]
fn Topbar() -> Html {
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

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub text: String,
    pub href: String,
}

#[function_component]
fn Button(props: &Props) -> Html {
    html! {
        <a href={props.href.clone()} class="relative inline-flex items-center justify-center px-6 py-3 overflow-hidden font-bold text-white rounded-md shadow-2xl group">
            <span class="absolute inset-0 w-full h-full transition duration-300 ease-out opacity-100 bg-gradient-to-br from-pink-600 via-purple-700 to-blue-400 group-hover:opacity-0"></span>
            <span class="absolute inset-0 w-full h-full transition duration-300 ease-out opacity-0 bg-gradient-to-br from-teal-600 to-emerald-600 group-hover:opacity-100"></span>
            <span class="relative">{props.text.clone()}</span>
        </a>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="bg-white dark:bg-black">
        <div class="pb-20 bg-gradient-to-r from-purple-900/30 to-teal-900/30 antialiased">
            <Topbar/>
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
            
            // get rid of this sometime lol
            <br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/>

        </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}