use yew::{function_component, html, Html};

use crate::components::*;

mod props;
use props::Props;

#[function_component]
pub fn Page(props: &Props) -> Html {
    html! {
        <div class="min-h-screen dark:bg-gradient-to-r dark:from-fuchsia-900/20 dark:to-pink-900/20">
            <Header />
            {for props.children.iter()}
            <Footer />
        </div>

    }
}
