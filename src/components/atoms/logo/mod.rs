use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Logo(props: &Props) -> Html {
    if props.style == "circle" {
        html! {
            <img src="images/logo/circle/1024.png" class="h-9 rounded-full" alt="Logo"/>
        }
    } else if props.style == "square" {
        //  props.style == "square"
        html! {
            <img src="images/logo/square/1024.png" class="h-9" alt="Logo"/>
        }
    } else if props.style == "labeled" {
        html! {
            <div class="flex justify-center gap-5">
                <Logo style="circle"/>
                <Label>
                    <p class="pt-0.5 font-bold text-xl invisible sm:visible">
                    {"Futino"}
                    </p>
                </Label>
            </div>
        }
    } else {
        html! {
            <Label>
            <p class="text-red-400">
            {"STYLE NOT SET FOR LOGO"}
            </p>
            </Label>
        }
    }
}
