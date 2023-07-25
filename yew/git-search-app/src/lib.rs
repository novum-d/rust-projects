use gloo::console::log;
use yew::{classes, html, Component, Context, Html};

pub struct App {
    value: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="hero min-h-screen">
                <div class="hero-content">
                    <div class="mockup-browser border bg-base-300">
                        <div class="mockup-browser-toolbar">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
                            <input type="text" class="input input-ghost w-full max-w-xs" />
                        </div>
                        <div class="flex justify-center px-4 py-16 bg-base-200">{"Hello!"}</div>
                    </div>
                </div>
            </div>
        }
        // <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
        // <p>{ self.value }</p>
        // <button class="btn">{"button"}</button>
    }
}
