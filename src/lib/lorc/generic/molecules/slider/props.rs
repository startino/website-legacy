use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::from(""))]
    pub input_id: String,
    #[prop_or(String::from(""))]
    pub label_id: String,
    pub min: String,
    pub max: String,
    #[prop_or(String::from("1"))]
    pub step: String,
}
