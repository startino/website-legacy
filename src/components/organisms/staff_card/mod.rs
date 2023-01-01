use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn StaffCard(props: &Props) -> Html {
    let Props {body, name, title} = props;

    html! {
        <div id="staff" class="py-20 px-4 sm:px-6 md:px-8 border-pink-500/40">
                <a href="#" class="flex flex-col items-center bg-white border rounded-lg shadow-md md:flex-row md:max-w-xl hover:bg-purple-100 dark:border-purple-700 dark:bg-purple-800 dark:hover:bg-purple-700">
                    <img class="object-cover w-full rounded-t-lg h-96 md:h-auto md:w-48 md:rounded-none md:rounded-l-lg" src="images/logo/square/1024.png" alt=""/>
                    <div class="flex flex-col justify-between p-4 leading-normal">
                        <Label>
                            <h5 class="mb-2 text-2xl">
                            {body}
                            </h5>
                        </Label>
                        <figcaption class=" align-text-bottom font-medium">
                            <div class="text-slate-300">
                            {title}
                            </div>
                            <div class="text-slate-400">
                            {name}
                            </div>
                        </figcaption>
                    </div>
                </a>
            </div>
    }
}