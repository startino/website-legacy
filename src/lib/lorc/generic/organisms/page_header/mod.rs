use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::lorc::generic::atoms::*;
use crate::lorc::generic::molecules::*;

#[function_component]
pub fn PageHeader(props: &Props) -> Html {
    html! {
        <div class="fixed top-0 z-40 flex-none w-full transition-colors duration-500 backdrop-blur bg-surface-light/5 dark:bg-surface-dark/5">
            <div class="py-4 mx-5 border-b border-secondary-light/10 dark:border-secondary-dark/10">
                <div class="flex relative items-center md:px-10 lg:px-20 xl:px-40 px-auto md:px-auto">
                    <a class="flex overflow-hidden px-3" href="/">
                        <Logo style="labeled" />
                    </a>
                    {for props.children.iter()}

                    <div class="flex items-center ml-auto">
                        <nav class="text-sm font-semibold leading-6 text-background-on-light dark:text-background-on-dark hover:text-primary-light dark:hover:text-primary-dark">
                            <ul class="flex m-auto space-x-8">
                                <li>
                                    <TextLink href="/">
                                        <Label size={SizeVariant::Medium}>
                                            {"Home"}
                                        </Label>
                                    </TextLink>
                                </li>
                                <li>
                                    <TextLink href="/contact">
                                        <Label size={SizeVariant::Medium}>
                                            {"Contact"}
                                        </Label>
                                    </TextLink>
                                </li>
                                <li>
                                    <TextLink href="/packages">
                                        <Label size={SizeVariant::Medium}>
                                            {"Packages"}
                                        </Label>
                                    </TextLink>
                                </li>
                            </ul>
                        </nav>
                    </div>
                </div>
            </div>
        </div>
    }
}
