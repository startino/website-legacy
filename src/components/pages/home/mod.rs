use yew::prelude::*;

use crate::components::*;

pub struct Home;

impl Component for Home {
type Message = ();
type Properties = ();

fn create(_ctx: &Context<Self>) -> Self {
    Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <main>
            <div class="py-40 px-4 sm:px-6 md:px-8 border-b dark:border-secondary-900/40 shadow-2xl">
                <div class="relative max-w-6xl mx-auto">
                    <Label>
                        <h1 class="font-extrabold text-4xl sm:text-5xl lg:text-6xl m-5">
                            {"Futino"}
                        </h1>
                    </Label>

                    <Label>
                        <h2 class="text-lg">
                            {"Dynamically built web-apps with "}
                            <a class="inline dark:text-primary-500 dark: hover:text-accent-500"
                                href="https://en.wikipedia.org/wiki/Free_and_open-source_software">
                                {"Free and Open-Source Software!"}
                            </a>
                        </h2>
                    </Label>

                    <div class="mt-6 sm:mt-10 flex justify-center space-x-6 text-sm">
                        <a href="/contact"><Button btn_type="button">
                                <p>{"Contact Us!"}</p>
                            </Button></a>
                        <a href="/about"><Button btn_type="button">
                                <p>{"Who Are We?"}</p>
                            </Button></a>
                    </div>
                </div>
            </div>

             // About us section: will include our staff cards and a brief introduction.
            <div class="justify-items-center my-10 border-b border-secondary-500/40 shadow-2xl">
                <div class="justify-items-center grid grid-cols-2">
                    <StaffCard body="This is some text that will be read but will be useless." name="Jonas Lewis"
                        title="Lead Developer, Co-Founder, & Co-Owner" />
                    <StaffCard body="This is some text that will be read but will be useless." name="Jorge Lindberg"
                        title="Lead Developer, Co-Founder, & Co-Owner" />
                </div>
                <div class="mx-auto max-w-2xl">
                    <Label>
                        <h3 class=" text-2xl p-6 ">
                            {"Futino creates and maintains dynamic web-apps that don't rely on proprietary
                            subscription-based solutions. We help growing companies and startups to build their presence
                            online with beautiful websites and apps that they can customise."}
                        </h3>
                        <h3 class="pt-3 p-5 ">
                            <a href="/about" class="text-primary-700 hover:text-accent-accent-dark">{"Learn more about us."}</a>
                        </h3>
                    </Label>
                </div>
            </div>

            // Client review section, will include the most valued reviews at the top.
            <div class="relative border-b dark:border-primary-500/40 shadow-2xl">
                <div class="max-w-4xl mx-auto">
                    <p class="text-4xl dark:text-white text-center  font-bold">
                        {
                        "How would you thrive without Futino?"
                        }
                    </p>
                    <p class="text-xl text-thin dark:text-white text-center p-6">
                        {
                        " \"The best solutions company on the planet. If you have a small business or are wanting to
                        start one,
                        using Futino's services is a must to compete in the space.\" "
                        }
                    </p>

                    <figure class="justify-center items-center mx-auto flex">
                        <img class="object-covet rounded-full w-12 h-12" src="images/logo/circle/1024.png" />
                        <figcaption class="text-left align-text-bottom font-medium p-3">
                            <p class="text-white">
                                {"CEO of "}
                                <a class="inline font-bold dark:text-primary-700 dark:hover:text-accent-accent-dark"
                                    href="https://www.microsoft.com">
                                    {"Apple"}
                                </a>
                            </p>

                            <p class="text-primary-700 dark:hover:text-accent-accent-dark">
                                <a href="https://en.wikipedia.org/wiki/Jeff_Bezos">
                                    {"Elon Musk"}
                                </a>
                            </p>
                        </figcaption>
                    </figure>
                </div>
                <div class="max-w-6xl mx-auto mt-10 grid grid-cols-3 gap-x-6">
                    <div class="row-end-auto grid grid-cols-1 gap-6">
                        <ClientReviewCard
                            body="Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."
                            name="Frog Mason" company="Orange" />
                        <ClientReviewCard body="blah blah some nice words about futino and how we're the best."
                            name="Frog Mason" company="Orange" />
                    </div>
                    <div class="grid grid-cols-1 gap-6">
                        <ClientReviewCard
                            body="Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit per morbi nullam metus nam class hac curabitur praesent sociosqu vulputate torquent sapien volutpat nostra vel curae consectetur curabitur."
                            name="Frog Mason" company="Orange" />
                        <ClientReviewCard body="Lorem ipsum dolor sit amet turpis quam netus imperdiet vel."
                            name="Frog Mason" company="Orange" />
                    </div>
                    <div class="grid grid-cols-1 gap-6">
                        <ClientReviewCard
                            body="Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."
                            name="Frog Mason" company="Orange" />
                        <ClientReviewCard body="blah blah some nice words about futino and how we're the best."
                            name="Frog Mason" company="Orange" />

                    </div>
                </div>
                <div class="p-10 flex justify-center text-sm">
                    <a href="#"><Button btn_type="button">
                            <p>{"Show more..."}</p>
                        </Button></a>
                </div>
            </div>

            <div class="relative border-b border-secondary-500/40 shadow-2xl">
                <div class="my-32 mx-32">
                    <Label>
                        <h1 class="text-left font-extrabold text-2xl">
                            {"A Solution To Your Obstacle."}
                        </h1>
                    </Label>
                    <Label>
                        <h1 class="text-left font-normal text-xl max-w-2xl pt-6">
                            {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                        </h1>
                    </Label>
                </div>
               

            </div>


        </main>

        }
        }
        }