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
            <a id="staffcard" href="#" class="flex flex-col items-center bg-white rounded-lg border shadow-md md:flex-row dark:border-primary-800 dark:bg-primary-800/20 dark:hover:bg-primary-800/30 hover:bg-primary-100">
                <img class="object-cover w-full h-96 rounded-t-lg md:w-48 md:h-auto md:rounded-none md:rounded-l-lg" src="images/logo/square/1024.png" alt=""/>
                <div class="box-content justify-between leading-normal">
                    <div class={format!("grid {} gap-y-3 p-4 leading-normal justify-between text-white font-light text-justify text-xl max-w-xl", cols)}>
                        {for children.iter()}
                    </div>


                    <figcaption class="p-4 font-medium align-text-bottom">
                        <div class="text-base text-white">
                        {title}
                        </div>
                        <div class="text-base text-slate-400">
                        {name}
                        </div>
                    </figcaption>

                </div>
            </a>
    }
}
