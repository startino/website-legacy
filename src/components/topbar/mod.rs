use yew::{function_component, html, Html, Properties, Children};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children, 
}

#[function_component]
pub fn Topbar(props: &Props) -> Html {
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