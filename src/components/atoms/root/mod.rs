use yew::{function_component, html, Html};

mod props;
use props::Props;

#[function_component]
pub fn Root(props: &Props) -> Html {
    html! {
        <div class="grow w-full h-full bg-gradient-to-r from-purple-900/30 to-teal-900/30 antialiased">
            {for props.children.iter()}
        </div>
    }
}
