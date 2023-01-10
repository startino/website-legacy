use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn StaffCard(props: &Props) -> Html {
    let Props {
        name,
        title,
        children,
    } = props;

    let cols = format!("grid-rows-{}", children.len());
    html! {
            <a id="staffcard" href="#" class="flex flex-col items-center bg-white border rounded-lg shadow-md md:flex-row hover:bg-primary-100 dark:border-primary-800 dark:bg-primary-800/20 dark:hover:bg-primary-800/30">
                <img class="object-cover w-full rounded-t-lg h-96 md:h-auto md:w-48 md:rounded-none md:rounded-l-lg" src="images/logo/square/1024.png" alt=""/>
                <div class="justify-between leading-normal box-content">
                    <div class={format!("grid {} gap-y-3 p-4 leading-normal justify-between text-white font-light text-justify text-xl max-w-xl", cols)}>
                        {for children.iter()}
                    </div>


                    <figcaption class="align-text-bottom font-medium p-4">
                        <div class="text-white text-base">
                        {title}
                        </div>
                        <div class="text-slate-400 text-base">
                        {name}
                        </div>
                    </figcaption>

                </div>
            </a>
    }
}
