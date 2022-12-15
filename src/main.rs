use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="py-20" style="background: linear-gradient(90deg, #667eea 0%, #764ba2 100%)">
            <nav>
                <img class="rounded-full mx-auto" src="./images/logo/circle/1024.png" alt="Logo" width="192" height="192"/>
            </nav>

            <div class="container mx-auto px-6">
                <h2 class="text-4xl font-bold mb-2 text-white">
                {"Futino"}
                </h2>
                <h3 class="text-2xl mb-8 text-gray-200">
                {"Dynamically built web-apps with Free Open Source Software!"}
                </h3>

                <button class="bg-white hover:bg-gray-100 font-bold rounded-full py-4 px-8 shadow-lg uppercase tracking-wider">
                {"Contact"}
                </button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}