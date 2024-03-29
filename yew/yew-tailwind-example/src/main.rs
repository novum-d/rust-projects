use yew::prelude::*;
fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <div class="min-h-screen bg-slate-500">
            <div class="flex">
                <ChatNotification />
                <ProfileCard />
            </div>
            <div class="flex">
                <NormalButton />
            </div>
        </div>
    }
}

#[function_component]
pub fn NormalButton() -> Html {
    html! {
        <button class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white">{"Save changes"}</button>
    }
}

#[function_component]
pub fn ChatNotification() -> Html {
    html! {
        <div class="p-6 max-w-sm mx-auto bg-white rounded-xl shadow-lg flex items-center space-x-4">
            <div class="shrink-0">
                <img class="h-12 w-12" src="assets/img/chat.svg" alt="ChitChat Logo" />
            </div>
            <div>
                <div class="text-xl font-medium text-black">{"ChitChat"}</div>
                <p class="text-slate-500">{"You have a new message!"}</p>
            </div>
        </div>
    }
}

#[function_component]
pub fn ProfileCard() -> Html {
    html! {
        <div
            class="py-8 px-8 max-w-sm mx-auto bg-white rounded-xl shadow-lg space-y-2 sm:py-4 sm:flex sm:items-center sm:space-y-0 sm:space-x-6">
            <img class="block mx-auto h-24 rounded-full sm:mx-0 sm:shrink-0" src="assets/img/erin-lindford.jpg" alt="Woman's Face" />
            <div class="text-center space-y-2 sm:text-left">
                <div class="space-y-0.5">
                    <p class="text-lg text-black font-semibold">
                        {"Erin Lindford"}
                    </p>
                    <p class="text-slate-500 font-medium">
                        {"Product Engineer"}
                    </p>
                </div>
                <button
                    class="px-4 py-1 text-sm text-purple-600 font-semibold rounded-full border border-purple-200 hover:text-white hover:bg-purple-600 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2">{"Message"}</button>
            </div>
        </div>
    }
}
