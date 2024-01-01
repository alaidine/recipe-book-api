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

pub async fn get_recipes(
    State(state): State<Arc<AppState>>
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res = match state
        .db
        .lock()
        .await
        .execute("SELECT * FROM recipes")
        .await
    {
        Ok(res) => res,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    Ok((StatusCode::OK, Json(res)))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_turso::Turso(
        addr="{secrets.TURSO_ADDR}",
        token="{secrets.TURSO_TOKEN}"
    )] client: Client,
    #[shuttle_secrets::Secrets] secret_store: SecretStore
) -> ShuttleAxum {
    client.batch([
        "CREATE TABLE IF NOT EXISTS recipes (name text, content text)"
    ])
    .await
    .unwrap();
    let db = Arc::new(Mutex::new(client));
    let state = Arc::new(AppState { db: db.clone() });
    let router = Router::new()
        .route("/", get(homepage))
        .route("/recipes", get(get_recipes))
        .with_state(state);

    Ok(router.into())
}