use crate::lorc::generic::*;
use yew::prelude::*;

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

                // Top section
                <div class="py-20 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
                    <div class="relative mx-auto max-w-6xl">
                        <Label>
                            <h1 class="m-5 text-4xl font-extrabold text-left">
                                {"Contact and FAQ"}
                            </h1>
                        </Label>
                    </div>
                </div>

                // Contact form section
                <div
                    class="grid relative grid-cols-4 gap-5 py-10 px-6 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
                    <div class="col-span-1 h-auto bg-surface-light dark:bg-surface-dark">
                        <Label>
                            <h3 class="m-5 text-4xl font-bold text-left text-surface-on-light dark:text-surface-on-dark">
                                {"Get in touch"}
                            </h3>
                        </Label>
                        <Label>
                            <h4 class="m-5 text-lg font-thin text-left">
                                {"Connect with Our Highly Skilled Team of Consultants for Personalized Solutions to Boost Your Business"}
                            </h4>
                        </Label>
                    </div>
                    <div class="col-span-3">
                        <ContactForm />
                    </div>
                </div>

                // FAQ section
                <div class="grid relative grid-cols-4 gap-5 py-10 px-6 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
                    <div class="col-span-1 h-auto bg-surface-light dark:bg-surface-dark">
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
                                        {"What is a website and why do I need one for my business?"}
                                    </h2>
                                </Label>
                                <Label>
                                    <h2 class="text-lg font-thin text-left">
                                        {"A website is an online representation of your business that
                                        can be accessed from anywhere in the world. Having a website
                                        allows you to showcase your products and services, reach a
                                        wider audience, and establish credibility for your business."}
                                    </h2>
                                </Label>
                            </li>
                            <li class="p-4">
                                <Label>
                                    <h2 class="text-2xl font-semibold text-left">
                                        {"Do you provide hosting and domain services?"}
                                    </h2>
                                </Label>
                                <Label>
                                    <h2 class="text-lg font-thin text-left">
                                        {"Yes, we provide hosting and domain services for your website
                                        starting at $20 per month for hosting and basic maintenance.
                                        Meaning we'll make sure the website stays up and implement
                                        updates at no extra charge."}
                                    </h2>
                                </Label>
                            </li>
                            <li class="p-4">
                                <Label>
                                    <h2 class="text-2xl font-semibold text-left">
                                        {"Can you help with search engine optimization (SEO)?"}
                                    </h2>
                                </Label>
                                <Label>
                                    <h2 class="text-lg font-thin text-left">
                                        {"Yes, we can help with search engine optimization (SEO) to
                                        improve the visibility of your website in search engine results
                                        pages (SERPs)."}
                                    </h2>
                                </Label>
                            </li>
                            <li class="p-4">
                                <Label>
                                    <h2 class="text-2xl font-semibold text-left">
                                        {"What happens if I need help after my website is completed?"}
                                    </h2>
                                </Label>
                                <Label>
                                    <h2 class="text-lg font-thin text-left">
                                        {"We provide ongoing support and maintenance services to
                                        ensure that your website is always up-to-date and functioning
                                        optimally. If you ever need assistance after your website is
                                        completed, simply contact our support team, and we will be
                                        happy to help."}
                                    </h2>
                                </Label>
                            </li>
                            <li class="p-4">
                                <Label>
                                    <h2 class="text-2xl font-semibold text-left">
                                        {"What is the typical timeline for building a website?"}
                                    </h2>
                                </Label>
                                <Label>
                                    <h2 class="text-lg font-thin text-left">
                                        {"The timeline for building a website depends on the scale and
                                        complexity of the project. For a standard website, the timeline can
                                        range from 4 to 8 weeks, while custom websites can take anywhere fro
                                        8 to 16 weeks. However, we understand the importance of a timely
                                        delivery and work efficiently to ensure your website is launched on
                                        time. With Futino you can expect a basic website to take under a week,
                                        and an advanced website to take less than a month. Of course this can
                                        expand if you have uniquely special needs."}
                                    </h2>
                                </Label>
                            </li>
                            <li class="p-4">
                                <Label>
                                    <h2 class="text-2xl font-semibold text-left">
                                        {"Can I make changes to my website after it is completed?"}
                                    </h2>
                                </Label>
                                <Label>
                                    <h2 class="text-lg font-thin text-left">
                                        {"Yes, you can make changes to your website after it is completed.
                                        We can provide ongoing support and maintenance to help you keep your
                                        website up-to-date."}
                                    </h2>
                                </Label>
                            </li>
                            <li class="p-4">
                                <Label>
                                    <h2 class="text-2xl font-semibold text-left">
                                        {"How much does it cost to build a website?"}
                                    </h2>
                                </Label>
                                <Label>
                                    <h2 class="text-lg font-thin text-left">
                                        {"When it comes to website development, the cost can vary depending
                                        on the scale and complexity of the project. The average cost of a
                                        standard website is between $2,000 and $5,000, while custom websites
                                        can range from $10,000 to $20,000. However, with Futino, you can
                                        receive a website that is both affordable and tailored to your needs,
                                        starting at under $400. This low cost offers a level of customization
                                        that exceeds expectations for a budget website. For those looking
                                        for a more advanced website, the cost is typically within the range
                                        of $2,000. Join the Futino family and experience personalized growth
                                        for your website. Begin with a basic foundation and evolve it over
                                        time with our flexible packages or tailored commission services."}
                                    </h2>
                                </Label>
                            </li>
                            <li class="p-4">
                                <Label>
                                    <h2 class="text-2xl font-semibold text-left">
                                        {"What is Rust, WebAssembly, Yew, and Tailwind CSS?"}
                                    </h2>
                                </Label>
                                <Label>
                                    <h2 class="text-lg font-thin text-left">
                                        {"Rust is a programming language that is designed to be fast,
                                        safe, and concurrent.
                                        WebAssembly is a low-level binary format that can be run in
                                        modern web browsers.
                                        Yew is a Rust library for building client-side web applications.
                                        Tailwind CSS is a utility-first CSS framework that makes it
                                        easy to create fast-loading and responsive web designs."}
                                    </h2>
                                </Label>
                            </li>
                        </ul>
                    </div>
                </div>
            </main>
        }
    }
}
