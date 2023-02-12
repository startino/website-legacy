use yew::{Children, Properties};

use crate::components::atoms::HeaderColor;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,

    // This is for setting the alignment, font-weight, and incase the user wants to override the padding of the header. *Padding is the only thing that I know of that can be overridden.*
    #[prop_or(String::from("text-center font-bold"))]
    pub class: String,
    #[prop_or(HeaderColor::OnBackground)]
    pub color: HeaderColor,
}
