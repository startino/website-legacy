use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn ContactForm(props: &Props) -> Html {
    html! {
        <form>
                        <div class="grid gap-6 mb-6 md:grid-cols-2">
                            <TextField label="first_name" title="Jorge" place_holder="First Name" required=true/>
                            <TextField label="last_name" title="Last Name" place_holder="Lewis" required=true/>
                            <TextField label="company_name" title="Company Name" place_holder="Futino" required=true/>
                            //
                        </div>

                        <BigTextField label="body" title="What would you like to say? :)" place_holder="I'd like to ask..." required=true/>

                        <div class="mb-6">
                        <TextField label="email" title="Email Address" place_holder="contact@futino.com" required=true/>
                        </div>

                        <button type="submit"
                            class="text-white text-center bg-accent-700 hover:bg-accent-800 focus:ring-4 focus:outline-none focus:ring-accent-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 ">
                            {"Submit"}
                        </button>
                    </form>
    }
}