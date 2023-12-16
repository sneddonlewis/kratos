extern crate diesel;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Compress, Logger, NormalizePath};

mod services;
mod schema;
mod models;
mod mail;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)] create_rust_app::setup_development().await;

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            .wrap(Logger::default());

        let mut api_scope = web::scope("/api");
        api_scope = api_scope.service(services::todo::endpoints(web::scope("/todos")));
        api_scope = api_scope.service(services::workout::endpoints(web::scope("/workouts")));

        #[cfg(debug_assertions)]
        {
            /* Development-only routes */
            
            /* Mount Swagger ui */
        }

        app = app.service(api_scope);
        app = app.default_service(
            actix_files::Files::new("/", "./frontend/dist/frontend/browser").index_file("index.html")
        );
        app
    }).bind("0.0.0.0:3000")?.run().await
}
