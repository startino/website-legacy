use yew::{Properties, Children};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub btn_type: String,
}