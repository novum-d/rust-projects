use askama::Template;
use axum::{
    extract::Form,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::{error::Error, process};
use std::{io::ErrorKind, net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| handler(&Note { title: "Note" })))
        .route(
            "/",
            post(|form: Form<SignUp>| {
                let sign_up = form.0;
                save_on_file(sign_up);
                handler(&Note { title: "Sign up!" })
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

fn save_on_file(sign_up: SignUp) {
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

fn write(file: File, sign_up: SignUp) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.serialize(sign_up)?;
    wtr.flush()?;
    Ok(())
}

#[derive(Template)]
#[template(path = "note.html")]
struct Note<'a> {
    title: &'a str,
}

#[derive(Serialize, Deserialize)]
struct SignUp {
    title: String,
    detail: String,
}
