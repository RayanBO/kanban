use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::{
    body::Body,
    extract::Json,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::json;
use tower::service_fn;


use crate::models::{Priority, Status};
use crate::store;

#[derive(Deserialize)]
struct MoveBody {
    id: String,
    status: String,
}

#[derive(Deserialize)]
struct AddBody {
    title: String,
    priority: Option<String>,
    assigned_to: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct IdBody {
    id: String,
}

async fn api_data() -> impl IntoResponse {
    match store::load() {
        Ok(data) => Json(data).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    }
}

async fn api_move(Json(body): Json<MoveBody>) -> impl IntoResponse {
    let new_status = match body.status.parse::<Status>() {
        Ok(s) => s,
        Err(e) => return (StatusCode::BAD_REQUEST, Json(json!({"error": e}))).into_response(),
    };
    let mut s = match store::load() {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    };
    let task = match s.tasks.iter_mut().find(|t| t.id == body.id) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, Json(json!({"error": "Tâche introuvable"}))).into_response(),
    };
    task.status = new_status;
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

async fn api_add(Json(body): Json<AddBody>) -> impl IntoResponse {
    let priority = body.priority.as_deref().unwrap_or("medium");
    let priority = match priority.parse::<Priority>() {
        Ok(p) => p,
        Err(e) => return (StatusCode::BAD_REQUEST, Json(json!({"error": e}))).into_response(),
    };
    let assigned_to = body.assigned_to.unwrap_or_default();
    let mut s = match store::load() {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    };
    let known_ids: Vec<&str> = s.users.iter().map(|u| u.id.as_str()).collect();
    for uid in &assigned_to {
        if !known_ids.contains(&uid.as_str()) {
            return (StatusCode::BAD_REQUEST, Json(json!({"error": format!("Utilisateur inconnu: {uid}")}))).into_response();
        }
    }
    let id = uuid::Uuid::new_v4().to_string();
    let task = crate::models::Task {
        id: id.clone(),
        title: body.title,
        priority,
        status: Status::Todo,
        assigned_to,
        created_at: chrono::Utc::now(),
        is_trash: false,
    };
    s.tasks.push(task);
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"id": id})).into_response()
}

async fn api_del(Json(body): Json<IdBody>) -> impl IntoResponse {
    let mut s = match store::load() {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    };
    let config = match store::load_config() {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    };
    let task = match s.tasks.iter_mut().find(|t| t.id == body.id) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, Json(json!({"error": "Tâche introuvable"}))).into_response(),
    };
    if config.use_trash {
        if task.is_trash {
            return (StatusCode::BAD_REQUEST, Json(json!({"error": "Déjà dans la corbeille"}))).into_response();
        }
        task.is_trash = true;
    } else {
        s.tasks.retain(|t| t.id != body.id);
    }
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

async fn api_folder() -> impl IntoResponse {
    let folder = std::env::current_dir()
        .unwrap_or_default()
        .file_name()
        .unwrap_or(std::ffi::OsStr::new("?"))
        .to_string_lossy()
        .to_string();
    Json(json!({"folder": folder}))
}

async fn api_init() -> impl IntoResponse {
    if store::is_initialized() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Déjà initialisé"}))).into_response();
    }
    let config = crate::models::Config::default();
    let s = crate::models::Store::default();
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    if let Err(e) = store::save_config(&config) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

async fn api_trash_restore(Json(body): Json<IdBody>) -> impl IntoResponse {
    let mut s = match store::load() {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    };
    let task = match s.tasks.iter_mut().find(|t| t.id == body.id && t.is_trash) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, Json(json!({"error": "Tâche introuvable dans la corbeille"}))).into_response(),
    };
    task.is_trash = false;
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

async fn api_trash_clean() -> impl IntoResponse {
    let mut s = match store::load() {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    };
    s.tasks.retain(|t| !t.is_trash);
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

pub fn find_port(start: u16) -> u16 {
    let mut p = start;
    loop {
        match TcpListener::bind(("127.0.0.1", p)) {
            Ok(listener) => {
                drop(listener);
                return p;
            }
            Err(_) => p += 1,
        }
    }
}

pub async fn run_server(port: u16, dashboard_dir: PathBuf) -> Result<(), String> {
    let api = Router::new()
        .route("/data", get(api_data))
        .route("/move", post(api_move))
        .route("/add", post(api_add))
        .route("/del", post(api_del))
        .route("/folder", get(api_folder))
        .route("/init", post(api_init))
        .route("/trash-restore", post(api_trash_restore))
        .route("/trash-clean", post(api_trash_clean));

    let dir = Arc::new(dashboard_dir);

    let index_html = Arc::new(
        std::fs::read_to_string(dir.join("index.html"))
            .unwrap_or_default()
    );

    let app = Router::new()
        .nest("/api", api)
        .fallback_service(service_fn(move |req: axum::http::Request<Body>| {
            let dir = dir.clone();
            let idx = index_html.clone();
            async move {
                let uri = req.uri().clone();
                let path = uri.path().trim_start_matches('/');
                let file_path = if path.is_empty() { dir.join("index.html") } else { dir.join(path) };

                let result: Result<Response<Body>, std::convert::Infallible> = match std::fs::read(&file_path) {
                    Ok(bytes) => {
                        let ct = guess_ct(&file_path);
                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header(axum::http::header::CONTENT_TYPE, ct)
                            .body(Body::from(bytes))
                            .unwrap())
                    }
                    Err(_) => {
                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")
                            .body(Body::from(idx.as_ref().clone()))
                            .unwrap())
                    }
                };
                result
            }
        }));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .map_err(|e| format!("Port {port}: {e}"))?;

    let url = format!("http://localhost:{}", port);
    println!("  Serveur lancé sur {}", url);

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Serveur: {e}"))
}

fn guess_ct(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("ico") => "image/x-icon",
        Some("json") => "application/json",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("ttf") => "font/ttf",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}
