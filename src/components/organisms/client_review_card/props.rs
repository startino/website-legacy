use yew::{Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub body: String,
    pub name: String,
    //pub title: String,
    //pub company_website: String,
    pub company: String,
}
