use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Footer(props: &Props) -> Html {
    html! {
        <div class="flex gap-5 justify-center mt-20 p-10 border-t border-secondary-300/10">
            <Logo style="labeled"/>
        </div>
    }
}
