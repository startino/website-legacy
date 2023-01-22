use yew::prelude::*;

use crate::components::*;

pub struct About;
impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <main>
          // Top section
          <div
            class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
            <div class="relative mx-auto max-w-5xl">
              <h1
                class="text-4xl font-extrabold tracking-tight text-center text-background-on-light dark:text-background-on-dark sm:text-5xl lg:text-6xl"
              >
                {"About Us"}
              </h1>

              <p class="mx-auto mt-6 max-w-3xl text-lg text-center text-background-on-light dark:text-background-on-dark">
                {"Who are we? what are we about? "}
                <a
                  class="inline text-primary-light dark:text-primary-dark hover:text-secondary-light dark:hover:text-secondary-dark"
                  href="#staff"
                >
                  {"Lets find out!"}
                </a>
              </p>
            </div>
          </div>

          // Staff section
          <div class="relative border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
            <div class="my-40 mx-32">
              <h1 class="p-6 text-3xl font-bold text-left text-background-on-light dark:text-background-on-dark">
                <a class="font-black text-primary-light">{"Who"}</a>
                {" we are."}
              </h1>
              <div id="staff" class="grid grid-cols-2 justify-items-center p-6">
                <StaffCard name="Jonas Lewis"
                        title="Lead Developer, Co-Founder, & Co-Owner" >
                        <p class="text-lg">
                            {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis
                            ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                            </p>
                            <p class="text-lg">
                                {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis 
                                ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                                </p>
                                <p class="text-lg">
                                  {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis
                                  ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                                  </p>
                        </StaffCard>
                        <StaffCard name="Jorge Lindberg"
                        title="Lead Developer, Co-Founder, & Co-Owner" >
                        <p>
                            {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis
                            ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                            </p>
                            <p>
                              {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis
                              ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                              </p>
                              <p>
                                {"Lorem ipsum dolor sit amet fermentum ut curabitur maecenas facilisis
                                ullamcorper ornare arcu amet dui habitasse placerat suspendisse vulputate nisl."}
                                </p>
                        </StaffCard>
              </div>
            </div>
          </div>

          // What we do
          <div class="relative border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
            <div class="my-32 mx-32">
              <h1 class="inline text-3xl font-bold text-left text-background-on-light dark:text-background-on-dark">
                <a class="font-black text-primary-light">{"What"}</a>
                {" we do."}
              </h1>
              <h1 class="pt-6 max-w-2xl text-xl font-normal text-left text-white">
                {"Futino creates and maintains dynamic web-apps that don't rely on
                proprietary subscription-based solutions. We help growing companies
                and startups to build their presence online with beautiful websites
                and apps that they can fully customise."}
              </h1>
            </div>
          </div>

          // Why we do
          <div class="relative border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40">
            <div class="my-32 mx-32">
              <h1 class="inline text-3xl font-bold text-left text-background-on-light dark:text-background-on-dark">
                <a class="font-black text-primary-light">{"Why"}</a>
                {" we do."}
              </h1>
              <h1 class="pt-6 max-w-2xl text-xl font-normal text-left text-background-on-light dark:text-background-on-dark">
                {"Futino creates and maintains dynamic web-apps that don't rely on
                proprietary subscription-based solutions. We help growing companies
                and startups to build their presence online with beautiful websites
                and apps that they can fully customise."}
              </h1>
            </div>
          </div>
        </main>
        }
    }
}
