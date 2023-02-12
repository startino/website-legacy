use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::atoms::HeaderColor;

/* Use docs
    <P class="text-center">
        {"More text than usual, usually around 20-200 words or smt, so like blah blih bleh."}
    </P>

    <P class="text-center" color={HeaderColor::OnSurface}>
        {"More text than usual, usually around 20-200 words or smt, so like blah blih bleh."}
    </P>
*/

#[function_component()]
pub fn P(props: &Props) -> Html {
    match props.color {
        HeaderColor::OnPrimary => {
            html! {
                <div class={format!("p-3 text-xl text-primary-on-light dark:text-primary-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
        HeaderColor::OnSecondary => {
            html! {
                <div class={format!("p-3 text-xl text-secondary-on-light dark:text-secondary-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
        HeaderColor::OnTertiary => {
            html! {
                <div class={format!("p-3 text-xl text-tertiary-on-light dark:text-tertiary-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
        HeaderColor::OnBackground => {
            html! {
                <div class={format!("p-3 text-xl text-background-on-light dark:text-background-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
        HeaderColor::OnSurface => {
            html! {
                <div class={format!("p-3 text-xl text-surface-on-light dark:text-surface-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
        HeaderColor::OnSurfaceVariant => {
            html! {
                <div class={format!("p-3 text-xl text-surface-variant-on-light dark:text-surface-variant-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
    }
}
