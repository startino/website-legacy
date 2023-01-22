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
            <div class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
                <div class="relative mx-auto max-w-6xl">
                    <Label>
                        <h1 class="m-5 text-4xl font-extrabold sm:text-5xl lg:text-6xl">
                            {"Futino"}
                        </h1>
                    </Label>

                    <Label>
                        <h2 class="text-lg">
                            {"Dynamically built web-apps with "}
                            <a class="inline text-tertiary hover:text-xl"
                                href="https://en.wikipedia.org/wiki/Free_and_open-source_software">
                                {"Free and Open-Source Software!"}
                            </a>
                        </h2>
                    </Label>

                    <div class="flex justify-center mt-6 space-x-6 text-sm sm:mt-10">
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
            <div class="justify-items-center my-10 border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
                <div class="grid grid-cols-2 justify-items-center">

                    <StaffCard name="Jonas Lewis"
                        title="Lead Developer, Co-Founder, & Co-Owner" >
                        <p>
                        {"This is some text that will be read but will be useless."}
                        </p>
                        </StaffCard>
                        <StaffCard name="Jonas Lewis"
                        title="Lead Developer, Co-Founder, & Co-Owner" >
                        <p>
                        {"This is some text that will be read but will be useless."}
                        </p>
                        </StaffCard>
                </div>
                <div class="mx-auto max-w-2xl align-middle text-center">
                    <Label>
                        <h3 class="p-6 text-2xl">
                            {"Futino creates and maintains dynamic web-apps that don't rely on proprietary
                            subscription-based solutions. We help growing companies and startups to build their presence
                            online with beautiful websites and apps that they can customise."}
                        </h3>
                    </Label>
                   
                        <h3 class="p-5 pt-3 inline-flex text-secondary-light dark:text-secondary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark">
                            <TextLink href="/about" text="Learn more about us."/><span><ExternalLinkIcon /></span>
                        </h3>
                    
                       
                </div>
            </div>

            // Client review section, will include the most valued reviews at the top.
            <div class="relative border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
                <div class="mx-auto max-w-4xl">
                    <p class="text-4xl font-bold text-center text-background-on-light dark:text-background-on-dark">
                        {
                        "How would you thrive without Futino?"
                        }
                    </p>
                    <p class="p-6 text-xl text-center text-background-on-light dark:text-background-on-dark text-thin">
                        {
                        " \"The best solutions company on the planet. If you have a small business or are wanting to
                        start one,
                        using Futino's services is a must to compete in the space.\" "
                        }
                    </p>

                    <figure class="flex justify-center items-center mx-auto">
                        <img class="w-12 h-12 rounded-full object-covet" src="images/logo/circle/1024.png" />
                        <figcaption class="p-3 font-medium text-left align-text-bottom">
                            <p class="text-white">
                                {"CEO of "}
                                <a class="inline font-bold text-primary-light dark:text-primary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark"
                                    href="https://www.microsoft.com">
                                    {"Apple"}
                                </a>
                            </p>

                            <p class="text-primary-light dark:text-primary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark">
                                <a href="https://en.wikipedia.org/wiki/Jeff_Bezos">
                                    {"Elon Musk"}
                                </a>
                            </p>
                        </figcaption>
                    </figure>
                </div>
                <div class="grid grid-cols-3 gap-x-6 mx-auto mt-10 max-w-6xl">
                    <div class="grid grid-cols-1 row-end-auto gap-6">
                        <ClientReviewCard
                            body="Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."
                            name="Frog Mason" company="Orange" title="CEO" website="https://www.apple.com/"/>
                        <ClientReviewCard body="blah blah some nice words about futino and how we're the best."
                            name="Frog Mason" company="Orange" title="CEO" website="https://www.apple.com/" />
                    </div>
                    <div class="grid grid-cols-1 gap-6">
                        <ClientReviewCard
                            body="Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit per morbi nullam metus nam class hac curabitur praesent sociosqu vulputate torquent sapien volutpat nostra vel curae consectetur curabitur."
                            name="Frog Mason" company="Orange" title="CEO" website="https://www.apple.com/"/>
                        <ClientReviewCard body="Lorem ipsum dolor sit amet turpis quam netus imperdiet vel."
                            name="Frog Mason" company="Orange" title="CEO" website="https://www.apple.com/"/>
                    </div>
                    <div class="grid grid-cols-1 gap-6">
                        <ClientReviewCard
                            body="Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."
                            name="Frog Mason" company="Orange" title="CEO" website="https://www.apple.com/"/>
                        <ClientReviewCard body="blah blah some nice words about futino and how we're the best."
                            name="Frog Mason" company="Orange" title="CEO" website="https://www.apple.com/"/>

                    </div>
                </div>
                <div class="flex justify-center p-10 text-sm">
                    <a href="#"><Button btn_type="button">
                            <p>{"Show more..."}</p>
                        </Button></a>
                </div>
            </div>

            <div class="relative border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
                <div class="my-32 mx-32">
                    <Label>
                        <h1 class="p-6 pl-0 text-3xl font-bold text-left text-background-on-light dark:text-background-on-dark">
                            {"A "}
                            <a class="font-black text-primary-light dark:text-primary-dark">{"Solution"}</a>
                            {" To Your Obstacle."}
                          </h1>
                    </Label>
                    <Label>
                        <h3 class="pt-6 max-w-2xl text-xl font-normal text-left">
                            {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                        </h3>
                    </Label>
                    <div class="text-sm pt-10">
                        <a href="#">
                            <Button btn_type="button">
                                <p class="inline-flex items-center gap-x-2">
                                    {"Learn more"}<span><RightArrow /></span>
                                </p>
                            </Button></a>
                    </div>
                </div>


            </div>


        </main>

        }
    }
}
