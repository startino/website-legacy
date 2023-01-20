
use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn TextField(props: &Props) -> Html {
    let Props { label, title, place_holder, required } = props;
    html! {
        <div>
                                <label for={label.to_owned()}
                                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{title}</label>
                                <input type="text" id={label.to_owned()}
                                    class="block p-2.5 w-full text-sm rounded-lg border bg-secondary-100 border-secondary-300 text-secondary-900 focus:ring-accent-500 focus:border-accent-500"
                                    placeholder={place_holder.to_owned()} required={required.to_owned()} />
                            </div>
    }
}
