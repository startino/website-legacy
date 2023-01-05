use yew::prelude::*;

use crate::components::*;

pub struct Contact;

impl Component for Contact {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <div class="py-40 px-4 sm:px-6 md:px-8 border-b border-pink-500/40 shadow-2xl">
                    <div class="relative max-w-6xl mx-auto ">
                        <Label>
                            <h1 class="font-extrabold text-4xl sm:text-5xl lg:text-6xl m-5">
                                {"Get in touch."}
                            </h1>
                        </Label>

                        <Label>
                            <h2 class="text-lg">
                                {"Dynamically built web-apps with "}
                                <a class="inline text-purple-400 hover:text-emerald-400"
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

                // Client review section, will include the most valued reviews at the top.
                <div class="relative max-w-6xl mx-auto ">
                    <div class="mt-20 grid grid-cols-3 gap-x-6">
                        <div class="row-end-auto grid grid-cols-1 gap-6">
                            <ClientReviewCard
                                body="Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."
                                name="Frog Mason" company="Orange" />
                            <ClientReviewCard body="blah blah some nice words about futino and how we're the best."
                                name="Frog Mason" company="Orange" />
                            <ClientReviewCard
                                body="Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit per morbi nullam metus nam class hac curabitur praesent sociosqu vulputate torquent sapien volutpat nostra vel curae consectetur curabitur."
                                name="Frog Mason" company="Orange" />
                        </div>
                        <div class="grid grid-cols-1 gap-6">
                            <ClientReviewCard
                                body="Lorem ipsum dolor sit amet enim amet ad eleifend consequat elit per morbi nullam metus nam class hac curabitur praesent sociosqu vulputate torquent sapien volutpat nostra vel curae consectetur curabitur."
                                name="Frog Mason" company="Orange" />
                            <ClientReviewCard body="Lorem ipsum dolor sit amet turpis quam netus imperdiet vel."
                                name="Frog Mason" company="Orange" />
                            <ClientReviewCard
                                body="Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."
                                name="Frog Mason" company="Orange" />
                            <ClientReviewCard body="blah blah some nice words about futino and how we're the best."
                                name="Frog Mason" company="Orange" />
                        </div>
                        <div class="grid grid-cols-1 gap-6">
                            <ClientReviewCard
                                body="Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."
                                name="Frog Mason" company="Orange" />
                            <ClientReviewCard body="blah blah some nice words about futino and how we're the best."
                                name="Frog Mason" company="Orange" />
                            <ClientReviewCard
                                body="Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."
                                name="Frog Mason" company="Orange" />
                        </div>
                    </div>


                </div>

            </main>

        }
    }
}
