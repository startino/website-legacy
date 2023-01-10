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

    let cols = format!("grid-cols-{}", children.len());
    html! {
        <div id="staff" class="py-6 px-4 sm:px-6 md:px-8 dark: border-secondary-600/40">
                <a href="#" class="flex flex-col items-center bg-white border rounded-lg shadow-md md:flex-row md:max-w-xl hover:bg-primary-100 dark:border-primary-800 dark:bg-primary-800/20 dark:hover:bg-primary-800/30">
                    <img class="object-cover w-full rounded-t-lg h-100 md:h-auto md:w-48 md:rounded-none md:rounded-l-lg" src="images/logo/square/1024.png" alt=""/>
                    <div class="flex flex-col justify-between leading-normal">
                        <div class={format!("grid {} space-x-5 justify-between p-4 leading-normal text-white font-light text-xl text-justify", cols)}>
                            {for children.iter()}
                        </div>
                       

                        <figcaption class=" align-text-bottom font-medium pl-4">
                            <div class="text-slate-300 text-base">
                            {title}
                            </div>
                            <div class="text-slate-400 text-base">
                            {name}
                            </div>
                        </figcaption>
                    </div>
                </a>
            </div>
    }
}
