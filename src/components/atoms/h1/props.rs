use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or(String::new())]
    pub color: String,
    #[prop_or(String::new())]
    pub size: String,
    #[prop_or(String::new())]
    pub class: String,
}
