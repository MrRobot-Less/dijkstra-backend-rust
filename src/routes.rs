use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::state::AppState;
use crate::models::Graph;

#[derive(Deserialize)]
pub struct SolveQuery {
    pub target: String,
}

pub async fn solve(data: web::Data<AppState>, query: web::Query<SolveQuery>) -> impl Responder {
    let stored_graph = data.graph.lock().unwrap();
    if let Some(graph) = &*stored_graph {
        let target_node = &query.target; // /solve?target=node

        if !graph.nodes.contains(target_node) {
            return HttpResponse::BadRequest().json("Target node not found in the graph");
        }

        
        let path_found = graph.solve_it(target_node);
        HttpResponse::Ok().json(path_found)
    } else {
        HttpResponse::BadRequest().json("No graph has been imported")
    }
}

// Import Graph Route
pub async fn import_graph(graph: web::Json<Graph>, data: web::Data<AppState>) -> impl Responder {
    let mut stored_graph = data.graph.lock().unwrap();
    *stored_graph = Some(graph.into_inner());
    HttpResponse::Ok().json("Graph imported successfully")
}