use yew::{Properties};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub text: String,
    pub href: String,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    html! {
        <a href={props.href.clone()} class="relative inline-flex items-center justify-center px-6 py-3 overflow-hidden font-bold text-white rounded-md shadow-2xl group">
            <span class="absolute inset-0 w-full h-full transition duration-300 ease-out opacity-100 bg-gradient-to-br from-pink-600 via-purple-700 to-blue-400 group-hover:opacity-0"></span>
            <span class="absolute inset-0 w-full h-full transition duration-300 ease-out opacity-0 bg-gradient-to-br from-teal-600 to-emerald-600 group-hover:opacity-100"></span>
            <span class="relative">{props.text.clone()}</span>
        </a>
    }
}