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
                <a href="#" class="flex flex-col items-center bg-white border rounded-lg shadow-md md:flex-row max-w-xl hover:bg-primary-100 dark:border-primary-800 dark:bg-primary-800/20 dark:hover:bg-primary-800/30">
                    <img class="object-contain w-full h-auto rounded-t-lg h-100 md:h-auto md:w-48 md:rounded-none md:rounded-l-lg" src="images/logo/square/1024.png" alt=""/>
                    <div class="justify-between leading-normal box-content">
                        <div class={format!("grid {} gap-x-3 px-4 leading-normal justify-between text-white font-light text-justify text-xl", cols)}>
                            {for children.iter()}
                        </div>
                       

                        <figcaption class="align-text-bottom font-medium py-5">
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
