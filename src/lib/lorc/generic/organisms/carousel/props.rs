use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub image_paths: Vec<String>,
    pub height: String, // "h-600"
}
