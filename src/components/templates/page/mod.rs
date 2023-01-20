use yew::{function_component, html, Html};

use crate::components::*;

mod props;
use props::Props;

#[function_component]
pub fn Page(props: &Props) -> Html {
    html! {
        <div class="min-h-screen dark:bg-gradient-to-r from-bg-secondary-class dark:to-bg-secondary-class">
            <Header />
            {for props.children.iter()}
            <Footer />
        </div>

    }
}
