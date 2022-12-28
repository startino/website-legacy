use yew::{function_component, html, Html};

mod props;
use props::Props;

#[function_component]
pub fn Logo(props: &Props) -> Html {
    if props.style == "circle" {
        html! {
            <img src="images/logo/circle/1024.png" class="h-9 rounded-full" alt="Logo"/>
        }
    } else {
        //  props.style == "square"
        html! {
            <img src="images/logo/square/1024.png" class="h-9" alt="Logo"/>
        }
    }
}
