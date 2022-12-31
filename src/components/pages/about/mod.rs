use yew::prelude::*;

use crate::components::*;

pub struct About;
impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
            <div class="py-40 px-4 sm:px-6 md:px-8 border-b border-pink-500/40 shadow-2xl">
                <div class="relative max-w-5xl mx-auto">
                    <h1 class="font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center text-white">
                        {"About Us"}
                    </h1>

                    <p class="mt-6 text-lg text-white text-center max-w-3xl mx-auto">
                        {"Who are we? what are we about? "}
                        <a class="inline text-purple-400 hover:text-emerald-400"
                            href="#staff">
                            {"Lets find out!"}
                        </a>
                    </p>
                </div>
            </div>

            <div id="staff" class="py-20 px-4 sm:px-6 md:px-8 border-b border-pink-500/40">
                <a href="#" class="flex flex-col items-center bg-white border rounded-lg shadow-md md:flex-row md:max-w-xl hover:bg-purple-100 dark:border-purple-700 dark:bg-purple-800 dark:hover:bg-purple-700">
                    <img class="object-cover w-full rounded-t-lg h-96 md:h-auto md:w-48 md:rounded-none md:rounded-l-lg" src="images/logo/square/1024.png" alt=""/>
                    <div class="flex flex-col justify-between p-4 leading-normal">
                        <Label>
                            <h5 class="mb-2 text-2xl">
                            {"This is a large quote!"}
                            </h5>
                        </Label>
                        <figcaption class=" align-text-bottom font-medium">
                            <div class="text-slate-300">
                            {"Lead"}
                            </div>
                            <div class="text-slate-400">
                            {"Jorge"}
                            </div>
                        </figcaption>
                    </div>
                </a>
            </div>
            </main>
        }
    }
}
