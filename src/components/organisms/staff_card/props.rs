use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub body: String,
    pub name: String,
    pub title: String
}
