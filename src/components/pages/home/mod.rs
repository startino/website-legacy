use yew::prelude::*;
use yew_router::prelude::*;

use crate::lorc::generic::{Icon, IconType, *};
use crate::{atoms::*, Route};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let navigator: Navigator = _ctx.link().navigator().unwrap();
        let go_contact_page =
            ButtonOptions::route_button(navigator, RouteType::Route(Route::Contact));

        let navigator: Navigator = _ctx.link().navigator().unwrap();
        let go_about_page = ButtonOptions::route_button(navigator, RouteType::Route(Route::About));

        html! {
        <>
            <div class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
                <div class="relative mx-auto max-w-6xl">
                    <Label>
                        <h1 class="m-5 text-4xl font-extrabold sm:text-5xl lg:text-6xl">
                            {"Futino"}
                        </h1>
                    </Label>

                    <Label>
                        <h2 class="text-lg">
                            {"Launch Your Business's Online Presence with Confidence And Trust"}
                        </h2>
                    </Label>

                    <div class="flex justify-center mt-6 space-x-6 text-sm sm:mt-10">
                        <a href="/contact">
                            <Button options={go_contact_page}>
                                <p>{"Contact Us!"}</p>
                            </Button>
                        </a>
                        <a href="/about">
                            <Button options={go_about_page.clone()}>
                                <p>{"Who Are We?"}</p>
                            </Button>
                        </a>
                    </div>
                </div>
            </div>

            // Why choose us section
            <div class="relative border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
                    <H1>
                        {"Why choose us?"}
                    </H1>
                    <h3 class="p-6 pl-0 text-xl text-center text-background-on-light dark:text-background-on-dark">
                        {"In today's advanced digital landscape, it is vital to have a professional, beautiful, and user-friendly website for your business.
                        We help save you "}
                        <span class="text-primary-light dark:text-primary-dark font-bold">{"Time"}</span>
                        {", "}
                        <span class="text-primary-light dark:text-primary-dark font-bold">{"Money"}</span>
                        {", and "}
                        <span class="text-primary-light dark:text-primary-dark font-bold">{"Talent"}</span>

                    </h3>
                <div class="mx-20 my-20 grid grid-cols-3 gap-x-7 [&>*]:rounded [&>*]:border-outline-light [&>*]:dark:border-outline-dark [&>*]:bg-surface-light [&>*]:dark:bg-surface-dark [&>*]:shadow-md [&>*]:flex [&>*]:flex-col">
                    <div class="">
                        <h2 class="p-6 pl-0 text-3xl font-bold text-center text-background-on-light dark:text-background-on-dark">
                            {"Time"}
                        </h2>
                        <p class="text-center text-xl text-surface-on-light dark:text-surface-on-dark py-4 px-10">
                            {
                                "Time is money, especially for startups. That's why at Futino, we prioritize speed and efficiency in everything we do. 
                                Our streamlined process and experienced team ensure that creating a custom, professional website for your startup is a fast and
                                hassle-free experience. Say goodbye to the endless meetings, multiple revisions, and long wait times of traditional website development."
                            }
                        </p>
                    </div>
                    <div class="">
                        <h2 class="p-6 pl-0 text-3xl font-bold text-center text-background-on-light dark:text-background-on-dark">
                            {"Money"}
                        </h2>
                        <p class="text-center text-xl text-surface-on-light dark:text-surface-on-dark py-4 px-10">
                            {
                                "Don't let the high costs of traditional website development hold your startup back.
                                With Futino, you'll get a custom, professional website that not only meets your needs, but exceeds your expectations. 
                                Our expert team uses a streamlined process to deliver a website that is both affordable and fast."
                            }
                        </p>
                    </div>
                    <div class="">
                        <h2 class="p-6 pl-0 text-3xl font-bold text-center text-background-on-light dark:text-background-on-dark">
                            {"Talent"}
                        </h2>
                        <p class="text-center text-xl text-surface-on-light dark:text-surface-on-dark py-4 px-10">
                            {
                                "Trust our skilled team to create a custom, professional website for your startup. 
                                Our web designers and developers have the expertise to deliver a visually stunning, functional, 
                                and user-friendly website that sets you apart. Choose Futino for a website that showcases your skills."
                            }
                        </p>
                    </div>
                </div>
            </div>

             // About us section: will include our staff cards and a brief introduction.
            <div class="justify-items-center my-10 border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
                <div class="grid grid-cols-2 justify-items-center">

                    <StaffCard name="Jonas Lewis" title="Lead Developer, Co-Founder, & Co-Owner" >
                        <p>
                            {"This is some text that will be read but will be useless."}
                        </p>
                    </StaffCard>
                    <StaffCard name="Jonas Lewis" title="Lead Developer, Co-Founder, & Co-Owner" >
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

                        <h3 class="p-5 pt-3 items-center inline-flex text-secondary-light dark:text-secondary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark">
                            <TextLink href="/about" text="Learn more about us."/>
                            <span>
                                <Icon icon={IconType::ExternalLinkIcon} height="12px" width="12px" color="currentColor"/>
                            </span>
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
                    <a href="#"><Button options={go_about_page.clone()}>
                            <p>{"Show more..."}</p>
                        </Button></a>
                </div>
            </div>

            // Our Work
            <div class="relative border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
                <div class="mx-20 my-20">
                    <h1 class="p-12 mx-10 text-8xl text-left font-bold text-background-on-light dark:text-background-on-dark">
                        {"Our Work"}
                    </h1>
                    <div class="grid grid-cols-2 gap-y-12 gap-x-5 first-letter:[&>*]:p-12 [&>*]:flex [&>*]:flex-col">
                        <div class="">
                            <iframe class="border-outline-light dark:border-outline-dark" height="600" width="400" src="https://www.doublepoint.studio/"/>
                            <h3 class="text-3xl text-left font-bold text-background-on-light dark:text-background-on-dark">
                                {"KebabCorner"}
                            </h3>
                        </div> 
                        <div class="">
                            <iframe class="border-outline-light dark:border-outline-dark" height="600" width="400" src="https://www.doublepoint.studio/"/>
                            <h3 class="text-3xl text-left font-bold text-background-on-light dark:text-background-on-dark">
                                {"Hema Gym"}
                            </h3>
                        </div> 
                        <div class="">
                            <div class="">
                                <img class="object-none object-top h-110 w-56 transition-transform duration-700 hover:-translate-y-full" decoding="async" width="440" height="4968" src="https://i0.wp.com/www.doublepoint.studio/wp-content/uploads/2021/09/lebootcamp-pageaccueil.png?fit=1400%2C4968&amp;ssl=1" loading="lazy"/>
                            </div>
                            
                            <h3 class="text-3xl text-left font-bold text-background-on-light dark:text-background-on-dark">
                                {"Little Brother"}
                            </h3>
                        </div>      
                    </div>
                </div>
            </div>

            // What we do
            <div class="relative border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
                <div class="my-32 mx-32">
                    <Label>
                        <h1 class="p-6 pl-0 text-3xl font-bold text-left text-background-on-light dark:text-background-on-dark">
                            {"A "}
                            <a class="font-black text-primary-light dark:text-primary-dark">{"Solution"}</a>
                            {" To Your Obstacle."}
                          </h1>
                    </Label>
                        <h3 class="pt-6 max-w-3xl text-xl font-normal text-left text-background-on-light dark:text-background-on-dark">
                            {"We develop full stack, open-source web-applications to build your business to most of it's potential.
                            We use the "}
                            <span class="text-primary-light dark:text-primary-dark font-bold">{"fastest"}</span>
                            {" and most "}
                            <span class="text-primary-light dark:text-primary-dark font-bold">{"secured"}</span>
                            {" programming languages while also having a workflow of "}
                            <span class="text-primary-light dark:text-primary-dark font-bold">{"pure efficiency"}</span>
                            {" using Tailwind CSS."}
                        </h3>
                        <div class="flex flex-row gap-x-5 items-center">
                            <crate::atoms::Icon icon={crate::atoms::IconType::RustLogo} height="64" width="64" color="red" />
                            <crate::atoms::Icon icon={crate::atoms::IconType::TailwindLogo} height="64" width="128" color="black" />
                        </div>
                    <div class="text-sm pt-10">
                            <Button options={go_about_page.clone()}>
                                <p class="group inline-flex items-center gap-x-2 ">
                                    {"Learn more"}
                                    <span class="transition-transform group-hover:translate-x-3">
                                        <Icon icon={IconType::RightArrow} height="12" width ="12" color="currentColor"/>
                                    </span>
                                </p>
                            </Button>
                    </div>
                </div>


            </div>
        </>
        }
    }
}
