use anyhow::Result;
use serde_json::json;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router, response::IntoResponse, http::StatusCode, Server,
};
use serde::Deserialize;

use crate::model::{TodoEntry, TodoList};

enum ServerError {
    InternalError { message: String },
    NoSuchEntry
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ServerError::InternalError { message } => 
                (StatusCode::INTERNAL_SERVER_ERROR, message),

            ServerError::NoSuchEntry =>
                (StatusCode::NOT_FOUND, "No such entry found".to_owned())
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

#[derive(Deserialize)]
struct ListParams {
    #[serde(default = "ListParams::default_from")]
    from: usize,
    #[serde(default = "ListParams::default_size")]
    size: usize
}
impl ListParams {
    fn default_from() -> usize { 0 }
    fn default_size() -> usize { 10 }
}

async fn list(
    params: Query<ListParams>,
    State(data): State<Arc<Mutex<TodoList>>>,
) -> Result<Json<Vec<TodoEntry>>, ServerError> {
    match data.lock() {
        Ok(d) => {
            let res: Vec<TodoEntry> = d.entries().iter()
                .skip(params.from)
                .take(params.size)
                .map(|e| e.clone())
                .collect();
            Ok(Json(res))
        }
        Err(err) => 
            Err(ServerError::InternalError { message: format!("Error locking data: {}", err).to_string() })
    }
}

#[derive(Deserialize)]
struct AddEntryParams {
    title: String,
    body: String
}

async fn add_entry(
    params: Query<AddEntryParams>,
    State(data): State<Arc<Mutex<TodoList>>>,
) -> Result<& 'static str, ServerError> {
    match data.lock() {
        Ok(mut d) => {
            let entry = TodoEntry { title: params.title.to_owned(), body: params.body.to_owned() };
            d.add_entry(entry);

            Ok("ok")
        }
        Err(err) => 
            Err(ServerError::InternalError { message: format!("Error locking data: {}", err).to_owned() })
    }
}

#[derive(Deserialize)]
struct RemoveEntryParams {
    index: usize
}

async fn remove_entry(
    params: Query<RemoveEntryParams>,
    State(data): State<Arc<Mutex<TodoList>>>,
) -> Result<& 'static str, ServerError> {
    match data.lock() {
        Ok(mut d) => {
            d.remove_entry(params.index).map_err(|_| ServerError::NoSuchEntry)?;

            Ok("ok")
        }
        Err(err) => 
            Err(ServerError::InternalError { message: format!("Error locking data: {}", err).to_owned() })
    }
}


pub async fn start_server(bind_address: &str, data: TodoList) -> Result<()> {
    let data = Arc::new(Mutex::new(data));

    let app = Router::new()
        .route("/list", get(list))
        .route("/add", get(add_entry))
        .route("/add", post(add_entry))
        .route("/remove", get(remove_entry))
        .with_state(data)
        ;


    let addr: SocketAddr = bind_address.parse()?;
    Ok(axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?)
}
