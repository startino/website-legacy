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
            <Root>
                <Navbar>
                    <a class="px-3 flex overflow-hidden" href="#">
                        <Logo style="circle"/>
                    </a>

                    <div class="flex items-center ml-auto">
                        <nav class="text-sm leading-6 font-semibold text-purple-400">
                            <ul class="m-auto flex space-x-8">
                                <li class="hover:text-emerald-400">
                                    <a href="/">{"Home"}</a>
                                </li>
                                <li class="hover:text-emerald-400">
                                    <a href="contact">{"Contact"}</a>
                                </li>
                                <li class="border-l border-pink-300/10"/>
                                <li class="hover:text-emerald-400">
                                    <ThemeButton/>
                                </li>
                            </ul>
                        </nav>
                    </div>
                </Navbar>

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
                    <div class="relative max-w-5xl mx-auto">
                        <figure class="m-20 max-w-2xl flex  rounded-xl bg-gradient-to-r from-teal-900/30 to-purple-900/30 antialiased">
                            <img class="object-cover w-48 h-auto rounded-l-xl rounded-r-none" src="images/logo/square/1024.png" width="600" height="800"/>

                            <div class="pt-6 md:p-8 text-center md:text-left space-y-4">
                                <blockquote>
                                    <p class="text-lg dark:text-white text-left max-w-3xl mx-auto">
                                    {"This is a large quote!"}
                                    </p>
                                </blockquote>
                                <figcaption class=" align-text-bottom font-medium">
                                    <div class="text-slate-300">
                                    {"Lead"}
                                    </div>
                                    <div class="text-slate-400">
                                    {"Jorge"}
                                    </div>
                                </figcaption>
                            </div>
                        </figure>
                    </div>
                </div>
            </Root>
        }
    }
}
