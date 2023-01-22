use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

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
    <div class="pt-6 text-left bg-gradient-to-r rounded-xl from-primary-light/20 to-primary-light/30 hover:from-tertiary-light/20 hover:to-tertiary-light/30 dark:from-primary-dark/20 dark:to-primary-dark/30 dark:hover:from-tertiary-dark/20 dark:hover:to-tertiary-dark/30 ">
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
