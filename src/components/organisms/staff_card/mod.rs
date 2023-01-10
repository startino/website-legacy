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
        <a href="#" class="flex flex-col items-center bg-white border rounded-lg shadow-md md:flex-row md:max-w-xl hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700">
    <img class="object-cover w-full rounded-t-lg h-96 md:h-auto md:w-48 md:rounded-none md:rounded-l-lg" src="/docs/images/blog/image-4.jpg" alt="">
    <div class="flex flex-col justify-between p-4 leading-normal">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white-</h5>
        <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order.</p>
    </div>
</a>
        <div id="staff" class="py-6 px-4  sm:px-6 md:px-8 dark: border-secondary-600/40">
                <a href="#" class="flex flex-col items-center bg-white border rounded-lg shadow-md md:flex-row hover:bg-primary-100 dark:border-primary-800 dark:bg-primary-800/20 dark:hover:bg-primary-800/30">
                    <img class="object-cover w-full h-auto rounded-t-lg h-100 md:h-auto md:w-48 md:rounded-none md:rounded-l-lg" src="images/logo/square/1024.png" alt=""/>
                    <div class="flex flex-col justify-between p-4 leading-normal">
                        <div class={format!("grid {} gap-x-3 px-4 leading-normal justify-between text-white font-light text-lg", cols)}>
                            {for children.iter()}
                        </div>
                       

                        <figcaption class="align-text-bottom font-medium text-base py-5">
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
