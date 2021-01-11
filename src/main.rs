// #[macro_use]
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,middleware
};
#[macro_use]
extern crate validator_derive;
use actix_redis::RedisSession;
use actix_session::Session;
use actix_cors::Cors;
// use actix_web::middleware::cors::Cors;
use actix_files::NamedFile;
use dotenv::dotenv;
use listenfd::ListenFd;
use log::{info, error};
use serde::{Deserialize, Serialize};
use std::env;
use rand::Rng;
use std::path::PathBuf;
mod logger;
mod routes;
mod models;
mod config;
mod db;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    dotenv().ok();
    let pool = config::db_pool(env::var("DATABASE_URL").expect("Database url missing")).await.expect("DB FAILURE");
    let mut listenfd = ListenFd::from_env();
    logger::setup_logger().expect("Could not setup logger");
    // let _www = env::var("WWW").expect("WWW not set");
    let mut server = HttpServer::new(move || {
        App::new()
        .wrap(RedisSession::new("127.0.0.1:6379", &private_key))
            // enable logger - always register actix-web Logger middleware last
        .wrap(middleware::Logger::default())
        .data(pool.clone())
        .wrap(
            Cors::new() // <- Construct CORS middleware builder
            //   .allowed_origin("http://localhost:3000")
              .allowed_methods(vec!["GET", "POST","PUT"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .allowed_header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS)
              .allowed_header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
              .allowed_header(http::header::ACCEPT)
              .max_age(3600)
              .finish())
            .route("/hey", web::get().to(manual_hello))
            .configure(routes::init_routes)
            .service(actix_files::Files::new("/", "./wwwroot").index_file("index.html"))
    });
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    
    info!("Starting server");
    server.run().await
}