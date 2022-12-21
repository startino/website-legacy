use yew::{Properties, Html};
use yew::prelude::*;

#[function_component]
fn Topbar() -> Html {
    html! {
        <div class="sticky top-0 z-40 w-full backdrop-blur flex-none transition-colors duration-500 lg:z-50 dark:border-pink-50/[0.06] bg-black/5 supports-backdrop-blur:bg-white/95">
            <div class="max-w-8xl mx-auto">
                <div class="py-4 border-b border-pink-900 dark:border-pink-300/10 mx-4">
                    <div class="relative flex items-center">
                        <a class="mr-3 flex-none w-[2.0625rem] overflow-hidden md:w-auto" href="#">
                            <img src="images/logo/circle/1024.png" class="h-6 mr-3 sm:h-9 rounded-full" alt="Logo" />
                        </a>

                        <div class="relative items-center ml-auto">
                            <nav class="text-sm leading-6 font-semibold text-pink-700 dark:text-pink-200">
                                <ul class="flex space-x-8">
                                    <li>
                                        <a class="hover:text-sky-500 dark:hover:text-sky-400" href="#">{"Nav Item"}</a>
                                    </li>
                                </ul>
                            </nav>
                        </div>
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
            <span class="absolute inset-0 w-full h-full transition duration-300 ease-out opacity-0 bg-gradient-to-br from-pink-600 via-purple-700 to-blue-400 group-hover:opacity-100"></span>
            //<!-- Top glass gradient -->
            <span class="absolute top-0 left-0 w-full bg-gradient-to-b from-white to-transparent opacity-5 h-1/3"></span>
            //<!-- Bottom gradient -->
            <span class="absolute bottom-0 left-0 w-full h-1/3 bg-gradient-to-t from-white to-transparent opacity-5"></span>
            //<!-- Left gradient -->
            <span class="absolute bottom-0 left-0 w-4 h-full bg-gradient-to-r from-white to-transparent opacity-5"></span>
            //<!-- Right gradient -->
            <span class="absolute bottom-0 right-0 w-4 h-full bg-gradient-to-l from-white to-transparent opacity-5"></span>
            <span class="absolute inset-0 w-full h-full border border-white rounded-md opacity-10"></span>
            <span class="absolute w-0 h-0 transition-all duration-300 ease-out bg-white rounded-full group-hover:w-56 group-hover:h-56 opacity-5"></span>
            <span class="relative">{props.text.clone()}</span>
        </a>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="pb-20 antialiased text-pink-500 dark:text-pink-400 bg-white dark:bg-pink-900" style="background: linear-gradient(90deg, #470245 0%, #002C25 100%)">
            <Topbar/>
            <div class="px-4 sm:px-6 md:px-8">
                <div class="relative max-w-5xl mx-auto">
                    <h1 class="text-pink-900 font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center dark:text-white">
                        {"Futino"}
                    </h1>

                    <p class="mt-6 text-lg text-pink-600 text-center max-w-3xl mx-auto dark:text-pink-400">
                        {"Dynamically built web-apps with "}
                        <a class="inline text-purple-500 dark:text-purple-400"
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

            <br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/><br/>

        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}