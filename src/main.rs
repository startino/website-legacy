use yew::{Properties, Html};
use yew::prelude::*;

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
        <div class="py-20" style="background: linear-gradient(90deg, #470245 0%, #002C25 100%)">
            <div class="px-4 sm:px-6 md:px-8">
                <div class="relative max-w-5xl mx-auto">
                    <h1 class="text-slate-900 font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center dark:text-white">
                        {"Futino"}
                    </h1>

                    <p class="mt-6 text-lg text-slate-600 text-center max-w-3xl mx-auto dark:text-slate-400">
                        {"Dynamically built web-apps with "}
                        <a class="inline text-purple-500 dark:text-purple-400"
                            href="https://en.wikipedia.org/wiki/Free_and_open-source_software">
                            {"Free and Open-Source Software!"}
                        </a>
                    </p>

                    <div class="mt-6 sm:mt-10 flex justify-center space-x-6 text-sm">
                        <Button text={"Contact"} href={"#_"}/>
                        <Button text={"Why choose us?"} href={"#_"}/>
                    </div>
                </div>
            </div>
        </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}