use yew::Properties;

use super::IconType;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub icon: IconType,
    pub height: String,
    pub width: String,
    #[prop_or_default]
    pub color: String,
}
