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
        // many <br/>, get rid of it when page actuall has content
        let brs = (0..999)
            .collect::<Vec<i32>>()
            .iter()
            .map(|_| html! {<br/>})
            .collect::<Html>();

        html! {
            <div class="bg-white dark:bg-black">
                <div class="pb-20 bg-gradient-to-r from-purple-900/30 to-teal-900/30 antialiased">
                    <Navbar>
                        <a class="px-3 flex overflow-hidden" href="#">
                            <Logo style="circle"/>
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
                    </Navbar>

                    <div class="py-40 px-4 sm:px-6 md:px-8 border-b border-pink-500/40 shadow-2xl">
                        <div class="relative max-w-5xl mx-auto">
                            <h1 class="font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center text-white">
                                {"About Us"}
                            </h1>

                            <p class="mt-6 text-lg text-white text-center max-w-3xl mx-auto">
                                {"Who are we, what are we about?"}
                                <a class="inline text-purple-400 hover:text-emerald-400"
                                    href="#">
                                    {"Lets find out!"}
                                </a>
                            </p>

                            <div class="mt-6 sm:mt-10 flex justify-center space-x-6 text-sm">
                                <a href="/contact"><Button btn_type="button"><p>{"Contact Us!"}</p></Button></a>
                                <a href="/about"><Button btn_type="button"><p>{"Who Are We?"}</p></Button></a>
                            </div>
                        </div>
                    </div>

                    <div class="py-20 px-4 sm:px-6 md:px-8 border-b border-pink-500/40">
                        <div class="relative max-w-5xl mx-auto">
                        </div>
                    </div>
                    // temp, get rid of when page has content
                    {brs}

                </div>
            </div>
        }
    }
}
