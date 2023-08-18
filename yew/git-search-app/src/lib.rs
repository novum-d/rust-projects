use openapi::apis::default_api::SearchRepositoriesGetError;
use openapi::apis::{configuration::Configuration, default_api::search_repositories_get, Error};
use openapi::models::Repo;
use state::{FetchState, State};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::html::Scope;
use yew::{
    classes, function_component, html, use_state, Component, Context, ContextProvider, Html,
    NodeRef, TargetCast,
};

mod state;

pub struct App {
    state: State,
    focus_ref: NodeRef,
}

pub enum Msg {
    SetReposFetchState(FetchState<Vec<Repo>>),
    Search(String),
    ChangeTheme,
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
            is_light_mode: false,
        };
        let focus_ref = NodeRef::default();
        Self { state, focus_ref }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetReposFetchState(fetch_state) => {
                self.state.entries = fetch_state;
                if let Some(input) = self.focus_ref.cast::<HtmlInputElement>() {
                    input.focus().unwrap();
                }
                true
            }
            Msg::Search(keyword) => {
                self.state.keyword = keyword.clone();
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
            Msg::ChangeTheme => {
                self.state.is_light_mode = !self.state.is_light_mode.clone();
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div data-theme={if self.state.is_light_mode {"light"} else {"dark"} }>
                 { self.view_header(ctx.link()) }
                 <main class="mt-16 py-12 flex justify-center">
                     <div class="flex flex-col main-clamp">
                         { self.view_keyword_input(ctx.link()) }
                         <div class="@container grow hidden-scrollbar">
                             { self.view_repo_list(ctx.link()) }
                         </div>
                     </div>
                 </main>
            </div>
        }
    }
}

impl App {
    fn view_header(&self, link: &Scope<Self>) -> Html {
        html! {
             <header class="fixed top-0 left-0 right-0 z-max">
                 <nav class="navbar bg-base-100 flex justify-between px-5 drop-shadow-md">
                     <a href="#" class="btn btn-ghost -m-1.5 p-1.5">
                         <svg class="fill-current w-8 h-8" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
                         <span class="normal-case text-xl">{"Github Search"}</span>
                     </a>
                     <label class="swap swap-rotate">
                         <input type="checkbox"
                             ref={self.focus_ref.clone()}
                             onclick={link.callback(|_| Msg::ChangeTheme)} />
                         <svg class="swap-on fill-current w-8 h-8" xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960">
                             <path
                                 d="M479.765-340Q538-340 579-380.765q41-40.764 41-99Q620-538 579.235-579q-40.764-41-99-41Q422-620 381-579.235q-41 40.764-41 99Q340-422 380.765-381q40.764 41 99 41Zm.235 60q-83 0-141.5-58.5T280-480q0-83 58.5-141.5T480-680q83 0 141.5 58.5T680-480q0 83-58.5 141.5T480-280ZM70-450q-12.75 0-21.375-8.675Q40-467.351 40-480.175 40-493 48.625-501.5T70-510h100q12.75 0 21.375 8.675 8.625 8.676 8.625 21.5 0 12.825-8.625 21.325T170-450H70Zm720 0q-12.75 0-21.375-8.675-8.625-8.676-8.625-21.5 0-12.825 8.625-21.325T790-510h100q12.75 0 21.375 8.675 8.625 8.676 8.625 21.5 0 12.825-8.625 21.325T890-450H790ZM479.825-760Q467-760 458.5-768.625T450-790v-100q0-12.75 8.675-21.375 8.676-8.625 21.5-8.625 12.825 0 21.325 8.625T510-890v100q0 12.75-8.675 21.375-8.676 8.625-21.5 8.625Zm0 720Q467-40 458.5-48.625T450-70v-100q0-12.75 8.675-21.375 8.676-8.625 21.5-8.625 12.825 0 21.325 8.625T510-170v100q0 12.75-8.675 21.375Q492.649-40 479.825-40ZM240-678l-57-56q-9-9-8.629-21.603.37-12.604 8.526-21.5 8.896-8.897 21.5-8.897Q217-786 226-777l56 57q8 9 8 21t-8 20.5q-8 8.5-20.5 8.5t-21.5-8Zm494 495-56-57q-8-9-8-21.375T678.5-282q8.5-9 20.5-9t21 9l57 56q9 9 8.629 21.603-.37 12.604-8.526 21.5-8.896 8.897-21.5 8.897Q743-174 734-183Zm-56-495q-9-9-9-21t9-21l56-57q9-9 21.603-8.629 12.604.37 21.5 8.526 8.897 8.896 8.897 21.5Q786-743 777-734l-57 56q-8 8-20.364 8-12.363 0-21.636-8ZM182.897-182.897q-8.897-8.896-8.897-21.5Q174-217 183-226l57-56q8.8-9 20.9-9 12.1 0 20.709 9Q291-273 291-261t-9 21l-56 57q-9 9-21.603 8.629-12.604-.37-21.5-8.526ZM480-480Z" />
                         </svg>
                         <svg class="swap-off fill-current w-8 h-8" xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960">
                             <path
                                 d="M480-120q-150 0-255-105T120-480q0-150 105-255t255-105q8 0 17 .5t23 1.5q-36 32-56 79t-20 99q0 90 63 153t153 63q52 0 99-18.5t79-51.5q1 12 1.5 19.5t.5 14.5q0 150-105 255T480-120Zm0-60q109 0 190-67.5T771-406q-25 11-53.667 16.5Q688.667-384 660-384q-114.689 0-195.345-80.655Q384-545.311 384-660q0-24 5-51.5t18-62.5q-98 27-162.5 109.5T180-480q0 125 87.5 212.5T480-180Zm-4-297Z" />
                         </svg>
                     </label>
                 </nav>
             </header>
        }
    }

    fn view_keyword_input(&self, link: &Scope<Self>) -> Html {
        let search = move |input: HtmlInputElement| Msg::Search(input.value());

        let onkeypress = link.batch_callback(move |e: KeyboardEvent| {
            (e.key() == "Enter").then(|| search(e.target_unchecked_into()))
        });

        html! {
            <div class="form-control items-center mb-10">
                 <input
                     type="text"
                     placeholder="Search…"
                     class="input input-bordered searchbar-clamp"
                     value={self.state.keyword.clone()}
                     {onkeypress}
                 />
            </div>
        }
    }

    fn view_repo_list(&self, _link: &Scope<Self>) -> Html {
        match &self.state.entries {
            FetchState::NotFetching => html! { "" },
            FetchState::Fetching => html! {
            <div class="grid place-content-center h-full">
                <span class="loading loading-dots loading-lg"></span>
            </div>
            },
            FetchState::Success(repos) => {
                let last_index = repos.len() - 1;
                html! {
                    for repos.iter().enumerate().map( |(i,repo)| {
                        let default = String::from("");
                        let avatar_url = repo.owner
                            .as_ref()
                            .and_then(|owner| owner.avatar_url.as_ref())
                            .unwrap_or(&default);

                        // style
                        let margin_bottom = if i == last_index { None } else { Some("mb-10") };
                        let classes = classes!(Some("card card-side bg-base-100 shadow-xl [&>*]:min-w-0 @container @xs:flex-col @3xl:flex-row"), margin_bottom);
                         html! {
                             <div class={classes}>
                                 <figure class="rounded-none avatar @3xl:w-48 @xs:rounded-l-lg @xs:w-full @xs:rounded-t-lg"><img src={avatar_url.clone()} alt="Movie" /></figure>
                                 <div class="card-body">
                                     <h2 class="card-title inline-block text-ellipsis overflow-hidden whitespace-nowrap">{repo.full_name.as_ref()}</h2>
                                     <p>{repo.language.as_ref()}</p>
                                     <div class="card-actions justify-end flex items-center  space-x-4">
                                        <div class="flex space-x-2">
                                         <svg class="svg-icon" xmlns="http://www.w3.org/2000/svg" height="48" viewBox="0 -960 960 960" width="48"><path d="m323-205 157-94 157 95-42-178 138-120-182-16-71-168-71 167-182 16 138 120-42 178ZM233-80l65-281L80-550l288-25 112-265 112 265 288 25-218 189 65 281-247-149L233-80Zm247-355Z"/></svg>
                                         <span>{repo.stargazers_count.as_ref()}</span>
                                        </div>
                                        <div class="flex space-x-2">
                                         <svg class="svg-icon" xmlns="http://www.w3.org/2000/svg" height="48" viewBox="0 -960 960 960" width="48"><path d="M372-120v-606l-90 90-42-42 162-162 162 162-42 42-90-90v320q30-33 71.5-54T607-481q16 0 38 3t41 8l-90-90 42-42 162 162-162 162-42-42 90-90q-16-5-38-8t-45-3q-57 0-104.5 34T432-290v170h-60Z"/></svg>
                                         <span>{repo.forks_count.as_ref()}</span>
                                        </div>
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
}
