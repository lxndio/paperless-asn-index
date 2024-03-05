mod endpoints;
mod paperless;

use std::collections::HashMap;

use actix_web::{get, post, web, App, HttpServer, Result};
use endpoints::{show_index, site};
use maud::{html, Markup};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            // .wrap(Logger::default())
            .service(actix_files::Files::new("/static", "./static"))
            .service(site)
            .service(show_index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
