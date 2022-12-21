use yew::{Properties, Html};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub text: String,
    pub href: String,
}

#[function_component]
fn Button(props: &Props) -> Html {
    html! {
        <a href={props.href.clone()} class="relative inline-flex items-center justify-center px-6 py-3 overflow-hidden font-bold text-white rounded-md shadow-2xl group">
            <span class="absolute inset-0 w-full h-full transition duration-300 ease-out opacity-0 bg-gradient-to-br from-pink-600 via-purple-700 to-blue-400 group-hover:opacity-100"></span>
            //<!-- Top glass gradient -->
            <span class="absolute top-0 left-0 w-full bg-gradient-to-b from-white to-transparent opacity-5 h-1/3"></span>
            //<!-- Bottom gradient -->
            <span class="absolute bottom-0 left-0 w-full h-1/3 bg-gradient-to-t from-white to-transparent opacity-5"></span>
            //<!-- Left gradient -->
            <span class="absolute bottom-0 left-0 w-4 h-full bg-gradient-to-r from-white to-transparent opacity-5"></span>
            //<!-- Right gradient -->
            <span class="absolute bottom-0 right-0 w-4 h-full bg-gradient-to-l from-white to-transparent opacity-5"></span>
            <span class="absolute inset-0 w-full h-full border border-white rounded-md opacity-10"></span>
            <span class="absolute w-0 h-0 transition-all duration-300 ease-out bg-white rounded-full group-hover:w-56 group-hover:h-56 opacity-5"></span>
            <span class="relative">{props.text.clone()}</span>
        </a>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
       
<nav class="bg-white border-gray-200 px-2 sm:px-4 py-2.5 rounded dark:bg-gray-900">
<div class="container flex flex-wrap items-center justify-between mx-auto">
  <a href="http://127.0.0.1:5500/dist/index.html" class="flex items-center">
      <img src="images/logo/circle/1024.png" class="h-6 mr-3 sm:h-9" alt="Logo" />
      <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white"> {"Home"}</span>
  </a>
  <button data-collapse-toggle="navbar-default" type="button" class="inline-flex items-center p-2 ml-3 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-default" aria-expanded="false">
    <span class="sr-only">{"Open main menu"}</span>
    <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path></svg>
  </button>
  <div class="hidden w-full md:block md:w-auto" id="navbar-default">
    <ul class="flex flex-col p-4 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
      <li>
        <a href="#" class="block py-2 pl-3 pr-4 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white" aria-current="page">{"Home"}</a>
      </li>
      <li>
        <a href="#" class="block py-2 pl-3 pr-4 text-gray-700 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">{"About"}</a>
      </li>
    </ul>
  </div>
</div>
<div class="scroll py-20" style="background: linear-gradient(90deg, #470245 0%, #002C25 100%)">
<div class="px-4 sm:px-6 md:px-8">
    <div class="relative max-w-5xl mx-auto">
        <h1 class="text-slate-900 font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center dark:text-white">
            {"Futino"}
        </h1>

        <p class="mt-6 text-lg text-slate-600 text-center max-w-3xl mx-auto dark:text-slate-400">
            {"Dynamically built web-apps with "}
            <a class="inline text-purple-500 dark:text-purple-400"
                href="https://en.wikipedia.org/wiki/Free_and_open-source_software">
                {"Free and Open-Source Software!"}
            </a>
        </p>

        <div class="mt-6 sm:mt-10 flex justify-center space-x-6 text-sm">
            <Button text={"Contact"} href={"#_"}/>
            <Button text={"Why choose us?"} href={"#_"}/>
        </div>
    </div>
</div>
</div>
</nav>




       
        

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}