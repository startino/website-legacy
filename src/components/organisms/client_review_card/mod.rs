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
    <div class="pt-6 text-left rounded-xl bg-gradient-to-r from-secondary-900/10 to-secondary-900/30 hover:from-accent-900/10 hover:to-accent-900/30">
    <a href={website.to_owned()}>
        <figure class="flex px-6">
            <img class="object-cover w-12 h-12 rounded-full" src="images/logo/square/512.png" width="600" height="800" />
            <figcaption class=" pl-3">
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
            <h3 class="text-left font-thin">

                {body}

            </h3>
            </Label>
        </blockquote>
    </a>
    </div>
    }
}
