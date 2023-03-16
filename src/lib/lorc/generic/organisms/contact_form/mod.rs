use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::lorc::generic::atoms::*;
use crate::lorc::generic::molecules::*;

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
                            class="py-2.5 px-5 w-full text-sm font-medium text-center rounded-lg sm:w-auto transition duration-300 text-secondary-on-light dark:text-secondary-on-dark hover:text-tertiary-on-light dark:hover:text-tertiary-on-dark bg-secondary-light dark:bg-secondary-dark hover:bg-tertiary-light dark:hover:bg-tertiary-dark">
                            {"Submit"}
                        </button>
                    </form>
    }
}
