use yew::{function_component, html, Html};

mod props;
use props::Props;


#[function_component]
pub fn Navbar(props: &Props) -> Html {
    html! {
        <div class="sticky z-40 top-0 w-full backdrop-blur flex-none transition-colors duration-500 bg-black/5">
            <div class="py-4 border-b border-pink-300/10 mx-5">
                <div class="px-auto xl:px-40 lg:px-20 md:px-10 md:px-auto relative flex items-center">
                    {for props.children.iter()}
                </div>
            </div>
        </div>
    }
}