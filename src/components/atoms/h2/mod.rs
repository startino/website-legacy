use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::atoms::HeaderColor;

/* Use docs
<H1 color=HeaderColor::OnBackground class="p-20 text-left font-superbold">
    {"Text here...."}
</H1>

<H2 color=HeaderColor::OnSurface class="text-right font-normal">
    {"Text here...."}
</H2>

<H3 color=HeaderColor::OnSurfaceVariant class="font-bold">
    {"Text here...."}
</H3>
*/

#[function_component()]
pub fn H2(props: &Props) -> Html {
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
                <div class={format!("p-6 text-3xl text-secondary-on-light dark:text-secondary-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
        HeaderColor::OnTertiary => {
            html! {
                <div class={format!("p-6 text-3xl text-tertiary-on-light dark:text-tertiary-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
        HeaderColor::OnBackground => {
            html! {
                <div class={format!("p-6 text-3xl text-background-on-light dark:text-background-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
        HeaderColor::OnSurface => {
            html! {
                <div class={format!("p-6 text-3xl text-surface-on-light dark:text-surface-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
        HeaderColor::OnSurfaceVariant => {
            html! {
                <div class={format!("p-6 text-3xl text-surface-variant-on-light dark:text-surface-variant-on-dark {}",props.class)}>
                    {for props.children.iter()}
                </div>
            }
        }
    }
}
