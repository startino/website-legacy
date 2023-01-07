use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub title: String,
    pub place_holder: String,
    pub required: bool,
}
