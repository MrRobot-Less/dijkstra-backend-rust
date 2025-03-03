use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use crate::routes::{import_graph, solve};
use crate::state::AppState;

mod models;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        graph: Arc::new(Mutex::new(None)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/import-graph", web::post().to(import_graph))
            .route("/solve", web::get().to(solve))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
