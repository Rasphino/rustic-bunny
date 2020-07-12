pub mod utils;
use actix_web::{
    web, App, HttpResponse, HttpServer, Responder,
    http::header
};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SearchRequest {
    cmd: String
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn search(web::Query(cmd): web::Query<SearchRequest>) -> impl Responder {
    let cmd = cmd.cmd;
    let command = utils::get_command_from_query_string(&cmd);
    let redirect_url = match command {
        "tw" => String::from("https://twitter.com"),
        "gh" => utils::github::construct_github_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd)
    };

    HttpResponse::TemporaryRedirect()
        .header(header::LOCATION, redirect_url)
        .finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/search", web::get().to(search))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
