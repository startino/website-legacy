use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::lorc::generic::atoms::*;
use crate::lorc::generic::molecules::*;

#[function_component]
pub fn StaffCard(props: &Props) -> Html {
    let Props {
        name,
        title,
        children,
    } = props;

    let cols = format!("grid-rows-{}", children.len());
    html! {
            <a id="staffcard" href="#" class="flex flex-col items-center rounded-lg border shadow-md md:flex-row bg-surface-light dark:bg-surface-dark hover:bg-primary-light/10 dark:hover:bg-primary-dark/10 border-primary-light dark:border-primary-dark ">
                <img class="object-cover w-full h-96 rounded-t-lg md:w-48 md:h-auto md:rounded-none md:rounded-l-lg" src="images/logo/square/1024.png" alt=""/>
                <div class="box-content justify-between leading-normal">
                    <div class={format!("grid {} gap-y-3 p-4 leading-normal justify-between font-light text-justify text-xl max-w-xl text-surface-on-light dark:text-surface-on-dark", cols)}>
                        {for children.iter()}
                    </div>


                    <figcaption class="p-4 font-medium align-text-bottom">
                        <div class="text-base text-surface-on-light dark:text-surface-on-dark">
                        {title}
                        </div>
                        <div class="text-base text-surface-on-light dark:text-surface-on-dark">
                        {name}
                        </div>
                    </figcaption>

                </div>
            </a>
    }
}
