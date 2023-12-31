#![allow(unused)]

use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    response::IntoResponse,
    Router,
    Json,
};
use shuttle_secrets::SecretStore;
use shuttle_axum::ShuttleAxum;
use libsql_client::client::Client;
use libsql_client::reqwest;
use anyhow::anyhow;
use tokio::sync::Mutex;
use std::sync::Arc;

struct Recipe {
    name: String,
    content: String,
}

async fn homepage() -> impl IntoResponse {
    r#"Welcome to the Recipe Book API!

Here are the following routes:
    - GET /recipes - Get all recipes.
    - GET /recipes/[name] - Get a specific recipe.
    - POST /recipes/create - Submit your own recipe 
        - Takes the following JSON parameters: "name" and "content"
"#
}

pub struct AppState {
    db: Arc<Mutex<Client>>,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_turso::Turso(
        addr="{secrets.DATABASE_URL}")] client: Client,
    #[shuttle_secrets::Secrets] secret_store: SecretStore
) -> ShuttleAxum {
    let db = Arc::new(Mutex::new(client));
    let state = Arc::new(AppState { db: db.clone() });
    let router = Router::new()
        .route("/", get(homepage))
        .with_state(state);

    Ok(router.into())
}