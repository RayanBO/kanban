use std::net::TcpListener;
use std::path::Path;

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

use crate::embed::DashboardAssets;
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

async fn api_config() -> impl IntoResponse {
    match store::load_config() {
        Ok(config) => Json(config).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    }
}

#[derive(Deserialize)]
struct ConfigUpdateBody {
    theme_dashboard: Option<String>,
    use_trash: Option<bool>,
}

async fn api_config_update(Json(body): Json<ConfigUpdateBody>) -> impl IntoResponse {
    let mut config = match store::load_config() {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    };
    if let Some(theme) = body.theme_dashboard {
        if theme != "dark" && theme != "light" {
            return (StatusCode::BAD_REQUEST, Json(json!({"error": "theme invalide"}))).into_response();
        }
        config.theme_dashboard = theme;
    }
    if let Some(use_trash) = body.use_trash {
        config.use_trash = use_trash;
    }
    if let Err(e) = store::save_config(&config) {
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

pub async fn run_server(port: u16) -> Result<(), String> {
    let api = Router::new()
        .route("/data", get(api_data))
        .route("/move", post(api_move))
        .route("/add", post(api_add))
        .route("/del", post(api_del))
        .route("/folder", get(api_folder))
        .route("/init", post(api_init))
        .route("/trash-restore", post(api_trash_restore))
        .route("/trash-clean", post(api_trash_clean))
        .route("/config", get(api_config).post(api_config_update));

    let app = Router::new()
        .nest("/api", api)
        .fallback_service(service_fn(|req: axum::http::Request<Body>| async move {
            let path = req.uri().path().trim_start_matches('/');
            let path = if path.is_empty() || path == "index.html" {
                "index.html"
            } else {
                path
            };

            let result: Result<Response<Body>, std::convert::Infallible> =
                match DashboardAssets::get(path) {
                    Some(asset) => {
                        let ct = guess_ct(path);
                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header(axum::http::header::CONTENT_TYPE, ct)
                            .body(Body::from(asset.data.to_vec()))
                            .unwrap())
                    }
                    None => {
                        match DashboardAssets::get("index.html") {
                            Some(asset) => Ok(Response::builder()
                                .status(StatusCode::OK)
                                .header(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")
                                .body(Body::from(asset.data.to_vec()))
                                .unwrap()),
                            None => Ok((StatusCode::NOT_FOUND, "Not Found").into_response()),
                        }
                    }
                };
            result
        }));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .map_err(|e| format!("Port {port}: {e}"))?;

    println!("  Serveur lancé sur http://localhost:{}", port);

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Serveur: {e}"))
}

fn guess_ct(path: &str) -> &'static str {
    let p = Path::new(path);
    match p.extension().and_then(|e| e.to_str()) {
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
