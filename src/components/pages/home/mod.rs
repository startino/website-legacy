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
                    <H1>
                        {"Futino"}
                    </H1>

                    <H3>
                        {"Launch Your Business's Online Presence with Confidence And Trust"}
                    </H3>

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
                    <div class="max-w-5xl mx-auto">
                        <H3>
                            {"In today's advanced digital landscape, it is vital to have a professional, beautiful, and user-friendly website for your business.
                            We help save you "}
                            <span class="text-primary-light dark:text-primary-dark font-bold">{"Time"}</span>
                            {", "}
                            <span class="text-primary-light dark:text-primary-dark font-bold">{"Money"}</span>
                            {", and "}
                            <span class="text-primary-light dark:text-primary-dark font-bold">{"Talent"}</span>

                        </H3>
                    </div>
                <div class="mx-20 my-20 grid grid-cols-3 gap-x-7 [&>*]:rounded [&>*]:border-outline-light [&>*]:dark:border-outline-dark [&>*]:bg-surface-light [&>*]:dark:bg-surface-dark [&>*]:shadow-md [&>*]:flex [&>*]:flex-col">
                    <div class="">
                        <H2>
                            {"Time"}
                        </H2>
                        <p class="text-center text-xl text-surface-on-light dark:text-surface-on-dark py-6 px-10">
                            {
                                "Time is money, especially for startups. That's why at Futino, we prioritize speed and efficiency in everything we do.
                                Our streamlined process and experienced team ensure that creating a custom, professional website for your startup is a fast and
                                hassle-free experience. Say goodbye to the endless meetings, multiple revisions, and long wait times of traditional website development."
                            }
                        </p>
                    </div>
                    <div class="">
                        <H2>
                            {"Money"}
                        </H2>
                        <p class="text-center text-xl text-surface-on-light dark:text-surface-on-dark py-6 px-10">
                            {
                                "Don't let the high costs of traditional website development hold your startup back.
                                With Futino, you'll get a custom, professional website that not only meets your needs, but exceeds your expectations. 
                                Our expert team uses a streamlined process to deliver a website that is both affordable and fast."
                            }
                        </p>
                    </div>
                    <div class="">
                        <H2>
                            {"Talent"}
                        </H2>
                        <p class="text-center text-xl text-surface-on-light dark:text-surface-on-dark py-6 px-10">
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
                        <H3>
                            {"Futino creates and maintains dynamic web-apps that don't rely on proprietary
                            subscription-based solutions. We help growing companies and startups to build their presence
                            online with beautiful websites and apps that they can customise."}
                        </H3>

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
                <div class="mx-64 my-20">
                    <H1 class="text-left font-bold p-12">
                        {"Our Work"}
                    </H1>
                    <div class="grid grid-cols-2 gap-y-24 gap-x-5">
                        <div class="place-self-start max-w-[500px]">
                            <iframe class="rounded-t-lg border-outline-light dark:border-outline-dark" height="600" width="500" src="https://www.doublepoint.studio/"/>
                            <div class="rounded-b-lg bg-gradient-to-r  from-primary-light/20 to-primary-light/30 hover:from-tertiary-light/20 hover:to-tertiary-light/30 dark:from-primary-dark/20 dark:to-primary-dark/30 dark:hover:from-tertiary-dark/20 dark:hover:to-tertiary-dark/30 ">
                                <a href="https://www.doublepoint.studio/" >
                                    <H3 class="text-left" color={HeaderColor::OnBackground}>
                                        {"Kebab Corner"}
                                    </H3>
                                    <p class="py-3 px-6 font-light text-xl text-background-on-light dark:text-background-on-dark">
                                        {"blah blah some nice words about futino and how we're the best and how we helped them x1000 they're business."}
                                    </p>
                                </a>
                            </div>
                        </div>
                        <div class="place-self-end max-w-[500px]">
                            <iframe class="rounded-t-lg border-outline-light dark:border-outline-dark" height="600" width="500" src="https://www.doublepoint.studio/"/>
                            <div class="rounded-b-lg bg-gradient-to-r from-primary-light/20 to-primary-light/30 hover:from-tertiary-light/20 hover:to-tertiary-light/30 dark:from-primary-dark/20 dark:to-primary-dark/30 dark:hover:from-tertiary-dark/20 dark:hover:to-tertiary-dark/30 ">
                                <a href="https://www.doublepoint.studio/" >
                                    <H3 class="text-left" color={HeaderColor::OnBackground}>
                                        {"Kebab Corner"}
                                    </H3>
                                    <p class="py-3 px-6 font-light text-xl text-background-on-light dark:text-background-on-dark">
                                        {"blah blah some nice words about futino and how we're the best and how we helped them x1000 they're business."}
                                    </p>
                                </a>
                            </div>
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

            // Contact section
            <div class="relative border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
                <div class="my-32 mx-32">
                    <div class="grid grid-cols-2 gap-x-10">
                        // Left side
                        <div>
                            <H1>
                                {"Unlock Your Startup's Potential"}
                            </H1>
                            <H3 class="max-w-3xl text-center">
                                {"Contact Us to Boost Your Startup's Potential"}
                            </H3>
                            <div class="p-12 grid grid-flow-col gap-x-5 auto-cols-max place-items-center">
                                // Office Location
                                <a class="flex flex-col place-items-center rounded-xl h-56 w-56 bg-surface-light dark:bg-surface-dark" href="/contact">
                                    <Icon class="p-3" icon={IconType::LocationIcon} height="64px" width="64px"/>
                                    <H3 class="py-2 text-center font-semibold" color={HeaderColor::OnSurface}>
                                        {"Main Office"}
                                    </H3>
                                    <H4 class="py-2 text-center font-light" color={HeaderColor::OnSurface}>
                                        {
                                            "Mui Wo, Ferry Pier 86, Hong Kong"
                                        }
                                    </H4>
                                </a>
                                // Phone number
                                <a class="flex flex-col place-items-center rounded-xl h-56 w-56 bg-surface-light dark:bg-surface-dark" href="/contact">
                                    <Icon class="p-3" icon={IconType::LocationIcon} height="64px" width="64px"/>
                                    <H3 class="py-2 text-center font-semibold" color={HeaderColor::OnSurface}>
                                        {"Phone Number"}
                                    </H3>
                                    <H4 class="py-2 text-center font-light" color={HeaderColor::OnSurface}>
                                        {
                                            "+852 9747 3013"
                                        }
                                    </H4>
                                </a>
                                // Email
                                <a class="flex flex-col place-items-center rounded-xl h-56 w-56 bg-surface-light dark:bg-surface-dark" href="/contact">
                                    <Icon class="p-3" icon={IconType::LocationIcon} height="64px" width="64px"/>
                                    <H3 class="py-2 text-center font-semibold" color={HeaderColor::OnSurface}>
                                        {"Email"}
                                    </H3>
                                    <H4 class="py-2 text-center font-light" color={HeaderColor::OnSurface}>
                                        {
                                            "contact@futino.com"
                                        }
                                    </H4>
                                </a>


                            </div>
                        </div>
                        // Right side
                        <ContactForm />
                    </div>
                </div>
            </div>
        </>
        }
    }
}
