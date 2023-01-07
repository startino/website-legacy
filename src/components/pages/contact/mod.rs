use yew::prelude::*; use crate::components::*; pub struct Contact; impl
Component for Contact { type Message = (); type Properties = (); fn create(_ctx:
&Context<Self>) -> Self { Self } fn view(&self, _ctx: &Context<Self>) -> Html { html! {
        <main>

            // Top section
            <div class="py-40 px-4 sm:px-6 md:px-8 border-b border-secondary-500/40 shadow-2xl">
                <div class="relative max-w-6xl mx-auto">
                    <Label>
                        <h1 class="text-left font-extrabold text-8xl m-5">
                            {"Get in touch."}
                        </h1>
                    </Label>

                    <Label>

                        <h2 class="text-left  text-xl m-5">
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
                class="relative grid grid-cols-4 gap-5 py-10 px-6 sm:px-6 md:px-8 border-b border-secondary-500/40 shadow-2xl">
                <div class="col-span-1 bg-primary-800/20 h-96">
                    <Label>
                        <h3 class="text-left font-bold text-4xl m-5">
                            {"Get in touch."}
                        </h3>
                    </Label>
                    <Label>
                        <h4 class="text-left font-thin text-lg m-5">
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
            class="relative grid grid-cols-4 gap-5 py-10 px-6 sm:px-6 md:px-8 border-b border-secondary-500/40 shadow-2xl">
            <div class="col-span-1 bg-primary-800/20 h-80">
                <Label>
                    <h3 class="text-left font-bold text-4xl m-5">
                        {"Frequently asked questions."}
                    </h3>
                </Label>
                <Label>
                    <h4 class="text-left font-thin text-lg m-5">
                        {"Can't find the answer you're looking for? "}
                        <a href="#" class="hover:text-accent-400">
                            {"Ask here."}
                        </a>
                    </h4>
                </Label>
            </div>
            <div class="col-span-3">
                <ul>
                    <li class="p-4">
                        <Label>
                            <h2 class="text-left font-semibold text-2xl">
                                {"What's on your bucket list this year?"}
                            </h2>
                        </Label>
                        <Label>
                            <h2 class="text-left font-thin text-lg">
                                {"Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit
                                per morbi nullam metus nam class hac curabitur praesent sociosqu
                                vulputate."}
                            </h2>
                        </Label>
                    </li>
                    <li class="p-4">
                        <Label>
                            <h2 class="text-left font-semibold text-2xl">
                                {"What's one place you've travelled that you never want to go back to?"}
                            </h2>
                        </Label>
                        <Label>
                            <h2 class="text-left font-thin text-lg">
                                {"Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit
                                per morbi nullam metus nam class hac curabitur praesent sociosqu
                                vulputate."}
                            </h2>
                        </Label>
                    </li>
                    <li class="p-4">
                        <Label>
                            <h2 class="text-left font-semibold text-2xl">
                                {"What's the worst movie you've ever seen?"}
                            </h2>
                        </Label>
                        <Label>
                            <h2 class="text-left font-thin text-lg">
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
