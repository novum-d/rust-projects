use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| "");
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
