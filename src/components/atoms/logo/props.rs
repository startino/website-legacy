use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub style: String,
}
