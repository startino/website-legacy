use gloo::console::log;
use yew::prelude::*;

use crate::lorc::generic::atoms::*;

mod props;
use props::Props;

pub enum Msg {
    Next,
    Prev,
}

/*
This component will be moved to lorc when done. Keeping it here for now for a faster workflow with git.
*/

pub struct Carousel {
    image_paths: Vec<String>,
    current_image: usize,
}
impl Component for Carousel {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let paths = &_ctx.props().image_paths;
        Self {
            image_paths: paths.to_vec(),
            current_image: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Next => {
                log!("Next");
                if self.current_image == self.image_paths.len() - 1 {
                    self.current_image = 0;
                } else {
                    self.current_image += 1;
                }
            }
            Msg::Prev => {
                log!("Prev");
                if self.current_image == 0 {
                    self.current_image = self.image_paths.len() - 1;
                } else {
                    self.current_image -= 1;
                }
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let on_prev = _ctx.link().callback(|_| Msg::Prev);
        let on_next = _ctx.link().callback(|_| Msg::Next);
        html! {
            <div class="relative w-full h-full">
                <button onclick={on_prev} class="absolute top-1/2 left-0 flex justify-center h-full px-4 group focus:outline-none">
                    <span class="inline-flex items-center justify-center w-10 h-10 rounded-full bg-primary-light/40 dark:bg-primary-dark/40 group-hover:bg-primary-light/60 dark:group-hover:bg-primary-dark/60 group-focus:ring-4 group-focus:ring-outline-light dark:group-focus:ring-outline-dark/70 group-focus:outline-none">
                        <Icon icon={IconType::LeftArrow} height="12" width="12" color="currentColor"/>
                    </span>
                </button>
                <button onclick={on_next} class="absolute top-1/2 right-0 flex justify-self-end justify-center h-full px-4 group focus:outline-none">
                    <span class="inline-flex items-center justify-center w-10 h-10 rounded-full bg-primary-light/40 dark:bg-primary-dark/40 group-hover:bg-primary-light/60 dark:group-hover:bg-primary-dark/60 group-focus:ring-4 group-focus:ring-outline-light dark:group-focus:ring-outline-dark/70 group-focus:outline-none">
                    <Icon icon={IconType::RightArrow} height="12" width="12" color="currentColor"/>
                    </span>
                </button>
            </div>
        }
    }
}
