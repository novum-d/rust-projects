use askama::Template;
use axum::{
    extract::Form,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::{error::Error, process};
use std::{io::ErrorKind, net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| handler(&PageTemplate { title: "IS41" })))
        .route(
            "/note",
            get(|| async {
                let list = read("note.csv").unwrap();
                let mut memo_list = Vec::new();
                for x in list.iter() {
                    memo_list.push(MemoTemplate {
                        title: &x.title,
                        detail: &x.detail,
                    });
                }
                let html = MemoListTemplate {
                    memo_list: &memo_list,
                }
                .render()
                .unwrap();

                Html(html).into_response()
            }),
        )
        .route(
            "/",
            post(|form: Form<Memo>| {
                let sign_up = form.0;
                save_on_file(sign_up);
                handler(&PageTemplate { title: "Sign up!" })
            }),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler<T: Template>(template: &T) -> impl IntoResponse {
    let html = template.render().unwrap();
    Html(html).into_response()
}

fn save_on_file(sign_up: Memo) {
    let path = "note.csv";
    let f = OpenOptions::new().append(true).open(path);
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create(path) {
            Ok(fc) => {
                println!("Created:: {}", path);
                fc
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        },
        Err(error) => {
            panic!("Error: {}", error);
        }
    };
    if let Err(e) = write(f, sign_up) {
        println!("{}", e);
        process::exit(1);
    }
}

fn write(file: File, sign_up: Memo) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.serialize(sign_up)?;
    wtr.flush()?;
    Ok(())
}

fn read(path: &str) -> Result<Vec<Memo>, Box<dyn Error>> {
    let mut r = ReaderBuilder::new().has_headers(false).from_path(path)?;
    let dc = r.deserialize::<Memo>();
    let mut memo_list = Vec::new();
    for d in dc {
        let memo = match d {
            Ok(m) => m,
            Err(e) => {
                panic!("Error: {}", e);
            }
        };
        memo_list.push(memo)
    }
    drop(r);
    Ok(memo_list)
}

#[derive(Template)]
#[template(path = "sign_up.html")]
struct PageTemplate<'a> {
    title: &'a str,
}

#[derive(Serialize, Deserialize)]
struct Memo {
    title: String,
    detail: String,
}

#[derive(Template)]
#[template(path = "memo.html")]
struct MemoListTemplate<'a> {
    memo_list: &'a Vec<MemoTemplate<'a>>,
}

struct MemoTemplate<'a> {
    title: &'a str,
    detail: &'a str,
}
