use yew::{function_component, html, Html};

mod props;
use props::Props;

#[function_component]
pub fn H1(props: &Props) -> Html {
    html! {
        <div class={format!("p-6 text-5xl text-center font-bold text-background-on-light dark:text-background-on-dark {}",props.class)}>
            {for props.children.iter()}
        </div>
    }
}