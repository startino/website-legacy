use yew::{function_component, html, Html};

mod props;
use props::Props;

#[function_component]
pub fn Button(props: &Props) -> Html {
    html! {
        <button type={props.btn_type.clone()} class="inline-flex overflow-hidden relative justify-center items-center py-3 px-6 font-bold text-on-ter rounded-md shadow-2xl group text-primary-on-light dark:text-primary-on-dark hover:text-secondary-light dark:hover:text-secondary-dark">
            <span class="absolute inset-0 w-full h-full bg-gradient-to-br from-primary-light to-secondary-light dark:from-primary-dark dark:to-secondary-dark opacity-100 transition duration-300 ease-out group-hover:opacity-0 "></span>
            <span class="absolute inset-0 w-full h-full bg-gradient-to-br from-secondary-light to-primary-light opacity-0 transition duration-300 ease-out group-hover:opacity-100 "></span>
            <span class="relative ">{for props.children.iter()}</span>
        </button>
    }
}
