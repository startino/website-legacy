use yew::{Properties, Html};
use yew::prelude::*;

pub mod topbar;
pub mod button;

use topbar::Topbar;
use button::Button;

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