use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, options, post},
    Extension, Json, Router,
};

use serde::{Deserialize, Serialize};
mod init;
mod internal;
mod tool;
mod util;
use init::static_json;
use internal::app0::controller::home::*;
use internal::app0::controller::index::*;
use internal::app0::controller::md::generate_md;
// use tower_http::services::ServeDir; 
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use std::path::Path;
use tokio::io::{self, AsyncReadExt};

// This will embed and serve files in the `src` directory and its subdirectories.

#[tokio::main]
async fn main() {
    // initialize tracing
    // let app0 = Router::new().nest("/static",ServeDir::new("static"));
    // let s=static_json::static_json_txt();
    // println!("{:?}",s);

    // build our application with a route
    let app = Router::new()
        .route("/navback/img0", get(index_nav_back))
        .route("/index", get(index_html))
        .route("/html/:name", get(render_html));

    // File is located relative to `CARGO_MANIFEST_DIR` (the directory containing the manifest of your package).
    // This will embed and serve the `README.md` file.

    // `POST /users` goes to `create_user`

    // run our app with hyper, listening globally on port 3000
    // let app = Router::new().nest("/app0", app1);//.nest("/app1", app1);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
