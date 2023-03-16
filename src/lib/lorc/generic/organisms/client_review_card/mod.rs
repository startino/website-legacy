use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::lorc::generic::atoms::*;
use crate::lorc::generic::molecules::*;

#[function_component]
pub fn ClientReviewCard(props: &Props) -> Html {
    let Props {
        body,
        name,
        title,
        website,
        company,
    } = props;
    let title_at_company = format!("{} of {}", title, company);
    html! {
    <div class="pt-6 text-left bg-gradient-to-r rounded-xl from-surface-light/70 to-surface-light/80 hover:from-tertiary-light/40 hover:to-tertiary-light/50 dark:from-surface-dark/70 dark:to-surface-dark/80 dark:hover:from-tertiary-dark/40 dark:hover:to-tertiary-dark/50 ">
    <a href={website.to_owned()}>
        <figure class="flex px-6">
            <img class="object-cover w-12 h-12 rounded-full" src="images/logo/square/512.png" width="600" height="800" />
            <figcaption class="pl-3">
            <Label>
            <h3 class="text-left">
            {
                name
            }
            </h3>
            </Label>
            <Label>
            <h3 class="text-left">
            {title_at_company}
            </h3>
            </Label>

            </figcaption>

        </figure>


        <blockquote class="p-6">
            <Label>
            <h3 class="font-thin text-left">

                {body}

            </h3>
            </Label>
        </blockquote>
    </a>
    </div>
    }
}
