use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::lorc::generic::atoms::*;

#[function_component]
pub fn TextField(props: &Props) -> Html {
    let Props {
        label,
        title,
        place_holder,
        required,
    } = props;
    html! {
        <div>
                                <label for={label.to_owned()}
                                    class="block mb-2 text-sm font-medium text-background-on-light dark:text-background-on-dark ">{title}</label>
                                <input type="text" id={label.to_owned()}
                                    class="block p-2.5 w-full text-sm rounded-lg border bg-tertiary-light border-outline-light text-tertiary-on-light focus:ring-tertiary-light focus:border-tertiary-light dark:bg-tertiary-dark dark:border-outline-dark dark:text-tertiary-on-dark dark:focus:ring-tertiary-dark dark:focus:border-tertiary-dark"
                                    placeholder={place_holder.to_owned()} required={required.to_owned()} />
                            </div>
    }
}
