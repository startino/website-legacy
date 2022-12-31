use yew::prelude::*;

use crate::components::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="py-40 px-4 sm:px-6 md:px-8 border-b border-pink-500/40 shadow-2xl">
                <div class="relative max-w-5xl mx-auto">
                    <Label>
                    <h1 class="font-extrabold text-4xl sm:text-5xl lg:text-6xl m-5">
                        {"Futino"}
                    </h1>
                    </Label>

                    <Label>
                        <h2 class="text-lg">
                        {"Dynamically built web-apps with "}
                            <a class="inline text-purple-400 hover:text-emerald-400"
                                href="https://en.wikipedia.org/wiki/Free_and_open-source_software">
                                {"Free and Open-Source Software!"}
                            </a>
                        </h2>
                    </Label>

                    <div class="mt-6 sm:mt-10 flex justify-center space-x-6 text-sm">
                        <a href="/contact"><Button btn_type="button"><p>{"Contact Us!"}</p></Button></a>
                        <a href="/about"><Button btn_type="button"><p>{"Who Are We?"}</p></Button></a>
                    </div>
                </div>
            </div>
        }
    }
}
