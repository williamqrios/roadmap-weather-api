use crate::models::AppState; 
use axum::{Router, routing::get}; 
use crate::handlers::*; 

/// Builds the router and the individual routes  
pub fn start(state: AppState) -> Router {


    let routes = Router::new()
        .route("/", get(hello))
        .with_state(state);

    routes 
}

