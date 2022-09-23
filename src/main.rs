use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpServer, Responder};

use crate::song::get_song_url;
mod song;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/url")]
async fn get_url() -> impl Responder {
    // format!("abc")
    let url_res = get_song_url(String::from("美人鱼"), String::from("林俊杰"));
    // println!("{}", url_res.await.unwrap());
    url_res.await.unwrap()
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // let cors = Cors::default()
        //     .allowed_origin("*")
        //     // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
        //     .allowed_methods(vec!["GET", "POST"])
        //     // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(3600);
        let cors = Cors::permissive();
        App::new().wrap(cors).service(greet).service(get_url)
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}
