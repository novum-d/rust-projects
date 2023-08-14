use openapi::apis::default_api::SearchRepositoriesGetError;
use openapi::apis::{configuration::Configuration, default_api::search_repositories_get, Error};
use openapi::models::Repo;
use state::{FetchState, State};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html, TargetCast};

mod state;

pub struct App {
    state: state::State,
}

pub enum Msg {
    SetReposFetchState(FetchState<Vec<Repo>>),
    Search(String),
}

async fn fetch_repos(keyword: &str) -> Result<Vec<Repo>, Error<SearchRepositoriesGetError>> {
    let config = Configuration::default();
    let q = Some(keyword.trim());
    let search_result = search_repositories_get(&config, q).await?;
    Ok(search_result.items.unwrap())
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let state = State {
            entries: FetchState::NotFetching,
            keyword: "".into(),
        };
        Self { state }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetReposFetchState(fetch_state) => {
                self.state.entries = fetch_state;
                true
            }
            Msg::Search(keyword) => {
                ctx.link().send_future(async move {
                    match fetch_repos(&keyword).await {
                        Ok(repos) => Msg::SetReposFetchState(FetchState::Success(repos)),
                        Err(err) => {
                            log::info!("{err}");
                            Msg::SetReposFetchState(FetchState::Failed(err))
                        }
                    }
                });
                ctx.link()
                    .send_message(Msg::SetReposFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <header>
                    <nav class="navbar bg-base-100 flex justify-between px-5 drop-shadow-md">
                        <a href="#" class="btn btn-ghost -m-1.5 p-1.5">
                            <img class="h-8 w-auto" src="static/assets/images/github-mark.svg" alt="" />
                            <span class="normal-case text-xl">{"Github Search"}</span>
                        </a>
                        <label class="swap swap-rotate">
                            <input type="checkbox" />
                            <svg class="swap-on fill-current w-8 h-8" xmlns="http://www.w3.org/2000/svg" height="48"
                                viewBox="0 -960 960 960" width="48">
                                <path
                                    d="M479.765-340Q538-340 579-380.765q41-40.764 41-99Q620-538 579.235-579q-40.764-41-99-41Q422-620 381-579.235q-41 40.764-41 99Q340-422 380.765-381q40.764 41 99 41Zm.235 60q-83 0-141.5-58.5T280-480q0-83 58.5-141.5T480-680q83 0 141.5 58.5T680-480q0 83-58.5 141.5T480-280ZM70-450q-12.75 0-21.375-8.675Q40-467.351 40-480.175 40-493 48.625-501.5T70-510h100q12.75 0 21.375 8.675 8.625 8.676 8.625 21.5 0 12.825-8.625 21.325T170-450H70Zm720 0q-12.75 0-21.375-8.675-8.625-8.676-8.625-21.5 0-12.825 8.625-21.325T790-510h100q12.75 0 21.375 8.675 8.625 8.676 8.625 21.5 0 12.825-8.625 21.325T890-450H790ZM479.825-760Q467-760 458.5-768.625T450-790v-100q0-12.75 8.675-21.375 8.676-8.625 21.5-8.625 12.825 0 21.325 8.625T510-890v100q0 12.75-8.675 21.375-8.676 8.625-21.5 8.625Zm0 720Q467-40 458.5-48.625T450-70v-100q0-12.75 8.675-21.375 8.676-8.625 21.5-8.625 12.825 0 21.325 8.625T510-170v100q0 12.75-8.675 21.375Q492.649-40 479.825-40ZM240-678l-57-56q-9-9-8.629-21.603.37-12.604 8.526-21.5 8.896-8.897 21.5-8.897Q217-786 226-777l56 57q8 9 8 21t-8 20.5q-8 8.5-20.5 8.5t-21.5-8Zm494 495-56-57q-8-9-8-21.375T678.5-282q8.5-9 20.5-9t21 9l57 56q9 9 8.629 21.603-.37 12.604-8.526 21.5-8.896 8.897-21.5 8.897Q743-174 734-183Zm-56-495q-9-9-9-21t9-21l56-57q9-9 21.603-8.629 12.604.37 21.5 8.526 8.897 8.896 8.897 21.5Q786-743 777-734l-57 56q-8 8-20.364 8-12.363 0-21.636-8ZM182.897-182.897q-8.897-8.896-8.897-21.5Q174-217 183-226l57-56q8.8-9 20.9-9 12.1 0 20.709 9Q291-273 291-261t-9 21l-56 57q-9 9-21.603 8.629-12.604-.37-21.5-8.526ZM480-480Z" />
                            </svg>
                            <svg class="swap-off fill-current w-8 h-8" xmlns="http://www.w3.org/2000/svg" height="48"
                                viewBox="0 -960 960 960" width="48">
                                <path
                                    d="M480-120q-150 0-255-105T120-480q0-150 105-255t255-105q8 0 17 .5t23 1.5q-36 32-56 79t-20 99q0 90 63 153t153 63q52 0 99-18.5t79-51.5q1 12 1.5 19.5t.5 14.5q0 150-105 255T480-120Zm0-60q109 0 190-67.5T771-406q-25 11-53.667 16.5Q688.667-384 660-384q-114.689 0-195.345-80.655Q384-545.311 384-660q0-24 5-51.5t18-62.5q-98 27-162.5 109.5T180-480q0 125 87.5 212.5T480-180Zm-4-297Z" />
                            </svg>
                        </label>
                    </nav>
                </header>
                <main class="mx-96 mt-24">
                    { self.view_entry_edit_input(&self.state.keyword,ctx.link()) }
                    { self.view_entry(ctx.link()) }
                </main>
            </>
        }
    }
}

impl App {
    fn view_entry(&self, _link: &Scope<Self>) -> Html {
        match &self.state.entries {
            FetchState::NotFetching => html! {"Yet"},
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(repos) => {
                let lastIndex = repos.len() - 1;
                html! {
                   for repos.iter().enumerate().map( |(i,repo)| {
                     let default = "".to_string();
                     let full_name = repo.full_name.as_ref().unwrap_or(&default);
                     let url = repo.owner.as_ref().and_then(|owner| owner.avatar_url.as_ref()).unwrap_or(&default);
                        let marginBottom = if i == lastIndex{ None }else{ Some("mb-10")};
                     let classes = classes!(Some("card card-side bg-base-100 shadow-xl"), marginBottom);
                     html! {
                         <div class={classes}>
                             <figure class="basis-3/12 avatar"><img class="object-cover" src={url.clone()} alt="Movie" /></figure>
                             <div class="card-body basis-8/12">
                                 <h2 class="card-title">{full_name}</h2>
                                 <p>{"Click the button to watch on Jetflix app."}</p>
                                 <div class="card-actions justify-end">
                                     <button class="btn btn-primary">{"Watch"}</button>
                                 </div>
                             </div>
                         </div>
                     }
                   })
                }
            }
            FetchState::Failed(err) => html! { err },
        }
    }

    fn view_entry_edit_input(&self, keyword: &str, link: &Scope<Self>) -> Html {
        let search = move |input: HtmlInputElement| Msg::Search(input.value());

        let onkeypress = link.batch_callback(move |e: KeyboardEvent| {
            (e.key() == "Enter").then(|| search(e.target_unchecked_into()))
        });

        html! {
            <div class="form-control px-36 mb-10">
                 <input
                     type="text"
                     placeholder="Search…"
                     class="input input-bordered flex-grow"
                     value={self.state.keyword.clone()}
                     {onkeypress}
                 />
            </div>
        }
    }
}
