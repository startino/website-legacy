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
            <div class="py-40 px-4 sm:px-6 md:px-8 border-b border-secondary-500/40 shadow-2xl">
                <div class="relative max-w-5xl mx-auto">
                    <h1 class="font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center text-white">
                        {"About Us"}
                    </h1>

                    <p class="mt-6 text-lg text-white text-center max-w-3xl mx-auto">
                        {"Who are we? what are we about? "}
                        <a class="inline text-primary-400 hover:text-emerald-400"
                            href="#staff">
                            {"Lets find out!"}
                        </a>
                    </p>
                </div>
            </div>
            <div class="my-10 mx-auto sm:px-6 md:px-8 border-b border-secondary-500/40 shadow-2xl">
                <StaffCard body="This is some text that will be read but will be useless."
                name="Jonas Lewis" title="Lead Developer, Co-Founder, & Co-Owner"/>
                <StaffCard body="This is some text that will be read but will be useless."
                name="Jorge Lindberg" title="Lead Developer, Co-Founder, & Co-Owner"/>
            </div>
            
            </main>
        }
    }
}
