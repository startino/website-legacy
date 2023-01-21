use yew::{function_component, html, Html};

use crate::components::*;

mod props;
use props::Props;

#[function_component]
pub fn Page(props: &Props) -> Html {
    html! {
        <div class="min-h-screen bg-gradient-to-r from-background-light/5 to-secondary-light/5 dark:from-background-dark/5 dark:to-secondary-dark/5">
            <Header />
            {for props.children.iter()}
            <Footer />
        </div>

    }
}
