
#[function_component]
pub fn ClientReviewCard(props: &Props) -> Html {
html! {
<div class="mt-20 text-left w-20 p-8 rounded-xl bg-gradient-to-r from-teal-900/30 to-purple-900/30 antialiased">
    <figure class="flex">
        <img class="object-cover w-12 h-12 rounded-full" src="images/pfp/jorge.jpg" width="600" height="800" />
        <figcaption class=" pl-20">
            <div class="text-white text-2xl font-normal">
                {&props.name}
            </div>
            <p class="text-white text-xl font-normal">
                {"CEO of "}
                <a class="inline text-purple-400 hover:text-emerald-400"
                    href="https://en.wikipedia.org/wiki/Free_and_open-source_software">
                    {&props.company}
                </a>
            </p>
        </figcaption>

    </figure>

    <blockquote class="p-8">
        <p class="text-lg text-white text-left font-thin">
            {
            &props.body
            }
        </p>
    </blockquote>

</div>
}
}