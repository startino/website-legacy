use yew::prelude::*; use crate::components::*; pub struct Contact; impl
Component for Contact { type Message = (); type Properties = (); fn create(_ctx:
&Context<Self>) -> Self { Self } fn view(&self, _ctx: &Context<Self>) -> Html { html! {
        <main>

            // Top section
            <div class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
                <div class="relative mx-auto max-w-6xl">
                    <Label>
                        <h1 class="m-5 text-8xl font-extrabold text-left">
                            {"Get in touch."}
                        </h1>
                    </Label>

                    <Label>

                        <h2 class="m-5 text-xl text-left">
                            {"Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit
                            per morbi nullam metus nam class hac curabitur praesent sociosqu
                            vulputate torquent sapien volutpat nostra vel curae consectetur
                            curabitur."}
                        </h2>
                    </Label>
                </div>
            </div>
            
            // Contact form section
            <div
                class="grid relative grid-cols-4 gap-5 py-10 px-6 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
                <div class="col-span-1 h-96 bg-surface-light dark:bg-surface-dark">
                    <Label>
                        <h3 class="m-5 text-4xl font-bold text-left text-surface-on-light dark:text-surface-on-dark">
                            {"Get in touch."}
                        </h3>
                    </Label>
                    <Label>
                        <h4 class="m-5 text-lg font-thin text-left">
                            {"Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit
                            per morbi nullam metus nam class hac curabitur praesent sociosqu
                            vulputate torquent sapien volutpat nostra vel curae consectetur
                            curabitur."}
                        </h4>
                    </Label>
                </div>
                <div class="col-span-3">
                    <ContactForm />
                </div>
            </div>

            // FAQ section
            <div
            class="grid relative grid-cols-4 gap-5 py-10 px-6 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
            <div class="col-span-1 h-80 bg-surface-light dark:bg-surface-dark">
                <Label>
                    <h3 class="m-5 text-4xl font-bold text-left">
                        {"Frequently asked questions."}
                    </h3>
                </Label>
                <Label>
                    <h4 class="m-5 text-lg font-thin text-left">
                        {"Can't find the answer you're looking for? "}
                        <a href="#" class="text-primary-light dark:text-primary-dark hover:text-secondary-light dark:hover:text-secondary-dark">
                            {"Ask here."}
                        </a>
                    </h4>
                </Label>
            </div>
            <div class="col-span-3">
                <ul>
                    <li class="p-4">
                        <Label>
                            <h2 class="text-2xl font-semibold text-left">
                                {"What's on your bucket list this year?"}
                            </h2>
                        </Label>
                        <Label>
                            <h2 class="text-lg font-thin text-left">
                                {"Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit
                                per morbi nullam metus nam class hac curabitur praesent sociosqu
                                vulputate."}
                            </h2>
                        </Label>
                    </li>
                    <li class="p-4">
                        <Label>
                            <h2 class="text-2xl font-semibold text-left">
                                {"What's one place you've travelled that you never want to go back to?"}
                            </h2>
                        </Label>
                        <Label>
                            <h2 class="text-lg font-thin text-left">
                                {"Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit
                                per morbi nullam metus nam class hac curabitur praesent sociosqu
                                vulputate."}
                            </h2>
                        </Label>
                    </li>
                    <li class="p-4">
                        <Label>
                            <h2 class="text-2xl font-semibold text-left">
                                {"What's the worst movie you've ever seen?"}
                            </h2>
                        </Label>
                        <Label>
                            <h2 class="text-lg font-thin text-left">
                                {"Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit
                                per morbi nullam metus nam class hac curabitur praesent sociosqu
                                vulputate."}
                            </h2>
                        </Label>
                    </li>
                </ul>

            </div>
        </div>
           
        </main>

        } } }
