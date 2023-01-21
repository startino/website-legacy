use yew::{function_component, html, Html};

mod props;
use props::Props;

#[function_component]
pub fn Label(props: &Props) -> Html {
    html! {
        <div class="text-center text-background-on-light dark:text-background-on-dark align-middle">
            {for props.children.iter()}
        </div>
    }
}
