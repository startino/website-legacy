use yew::{function_component, html, Html};

mod props;
use props::Props;

#[function_component]
pub fn Button(props: &Props) -> Html {
    html! {
        <button type={props.btn_type.clone()} class="inline-flex overflow-hidden relative justify-center items-center py-3 px-6 font-bold text-white rounded-md shadow-2xl group">
            <span class="absolute inset-0 w-full h-full bg-gradient-to-br to-blue-400 opacity-100 transition duration-300 ease-out group-hover:opacity-0 from-secondary-600 via-primary-700"></span>
            <span class="absolute inset-0 w-full h-full bg-gradient-to-br from-teal-600 opacity-0 transition duration-300 ease-out group-hover:opacity-100 to-accent-600"></span>
            <span class="relative">{for props.children.iter()}</span>
        </button>
    }
}
