use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub name: String,
    pub title: String,
    pub children: Children,
}
