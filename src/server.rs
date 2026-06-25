use std::net::TcpListener;
use std::path::Path;
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::task::{Context, Poll};

use axum::{
    body::Body,
    extract::{Json, Path as AxumPath, State},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::json;
use futures_core::Stream;
use tokio::sync::{broadcast, mpsc};
use tower::service_fn;

use crate::embed::DashboardAssets;
use crate::commands::export::{self, ExportFormat};
use crate::models::{Comment, Priority, Status, Store, User};
use crate::tags::normalize_tags;
use crate::store;

struct AppState {
    store: Mutex<Store>,
    events: broadcast::Sender<()>,
}

struct EventStream {
    rx: mpsc::Receiver<Result<Event, Infallible>>,
}

impl EventStream {
    fn new(rx: mpsc::Receiver<Result<Event, Infallible>>) -> Self {
        Self { rx }
    }
}

fn reload_event() -> Event {
    Event::default().event("reload").data("reload")
}

impl Stream for EventStream {
    type Item = Result<Event, Infallible>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

fn file_signature() -> (Option<SystemTime>, Option<SystemTime>) {
    let store_mtime = std::fs::metadata(store::data_path())
        .and_then(|m| m.modified())
        .ok();
    (store_mtime, None)
}

fn reload_state(state: &AppState) -> Result<(), String> {
    let store_data = store::load()?;
    *state.store.lock().unwrap() = store_data;
    Ok(())
}

async fn watch_files(state: Arc<AppState>) {
    let mut previous = file_signature();
    let mut interval = tokio::time::interval(Duration::from_millis(750));

    loop {
        interval.tick().await;
        let next = file_signature();
        if next != previous {
            previous = next;
            if reload_state(&state).is_ok() {
                let _ = state.events.send(());
            }
        }
    }
}

async fn api_events(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut b_rx = state.events.subscribe();
    let (tx, rx) = mpsc::channel(8);
    tokio::spawn(async move {
        loop {
            match b_rx.recv().await {
                Ok(_) | Err(broadcast::error::RecvError::Lagged(_)) => {
                    if tx.send(Ok(reload_event())).await.is_err() {
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });
    let stream = EventStream::new(rx);
    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}

#[derive(Deserialize)]
struct EditBody {
    id: String,
    title: Option<String>,
    priority: Option<String>,
    tags: Option<Vec<String>>,
    due_date: Option<String>,
}

#[derive(Deserialize)]
struct MoveBody {
    id: String,
    status: String,
}

#[derive(Deserialize)]
struct AddBody {
    title: String,
    priority: Option<String>,
    tags: Option<Vec<String>>,
    assigned_to: Option<Vec<String>>,
    due_date: Option<String>,
}

#[derive(Deserialize)]
struct IdBody {
    id: String,
}

#[derive(Deserialize)]
struct UserCreateBody {
    username: String,
    pic: Option<String>,
}

#[derive(Deserialize)]
struct UserUpdateBody {
    id: String,
    username: Option<String>,
    pic: Option<String>,
}

#[derive(Deserialize)]
struct AssignBody {
    task_id: String,
    assigned_to: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct CommentCreateBody {
    task_id: String,
    content: String,
    author_id: Option<String>,
}

async fn api_data(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let store = state.store.lock().unwrap();
    Json(&*store).into_response()
}

async fn api_export(
    State(state): State<Arc<AppState>>,
    AxumPath(format): AxumPath<String>,
) -> impl IntoResponse {
    let format = match format.as_str() {
        "json" => ExportFormat::Json,
        "md" | "markdown" => ExportFormat::Markdown,
        _ => {
            return (StatusCode::BAD_REQUEST, Json(json!({"error": "Format d'export invalide"})))
                .into_response();
        }
    };

    let store = state.store.lock().unwrap().clone();
    match export::write_store_export(&store, format, None) {
        Ok(path) => Json(json!({
            "ok": true,
            "format": format.label(),
            "path": path.to_string_lossy(),
        }))
        .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    }
}

async fn api_export_download(
    State(state): State<Arc<AppState>>,
    AxumPath(format): AxumPath<String>,
) -> impl IntoResponse {
    let format = match format.as_str() {
        "json" => ExportFormat::Json,
        "md" | "markdown" => ExportFormat::Markdown,
        _ => {
            return (StatusCode::BAD_REQUEST, Json(json!({"error": "Format d'export invalide"})))
                .into_response();
        }
    };

    let store = state.store.lock().unwrap().clone();
    let content = match export::render_store_export(&store, format) {
        Ok(content) => content,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response(),
    };

    let path = export::default_export_path(format);
    if let Err(e) = export::write_export_content(&path, &content) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, export::content_type(format))
        .header(
            header::CONTENT_DISPOSITION,
            format!(
                r#"attachment; filename="{}""#,
                export::download_filename(format)
            ),
        )
        .body(Body::from(content))
        .unwrap()
        .into_response()
}

async fn api_task_update(State(state): State<Arc<AppState>>, Json(body): Json<EditBody>) -> impl IntoResponse {
    if body.title.is_none()
        && body.priority.is_none()
        && body.tags.as_ref().is_none_or(|tags| tags.is_empty())
        && body.due_date.is_none()
    {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Spécifie au moins title, priority, tags ou due_date."}))).into_response();
    }
    let mut s = state.store.lock().unwrap();
    let task = match s.tasks.iter_mut().find(|t| t.id == body.id) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, Json(json!({"error": "Tâche introuvable"}))).into_response(),
    };
    if let Some(title) = body.title {
        task.title = title;
    }
    if let Some(priority) = body.priority {
        let p = match priority.parse::<Priority>() {
            Ok(p) => p,
            Err(e) => return (StatusCode::BAD_REQUEST, Json(json!({"error": e}))).into_response(),
        };
        task.priority = p;
    }
    if let Some(tags) = body.tags {
        task.tags = normalize_tags(tags);
    }
    if let Some(due) = body.due_date {
        task.due_date = if due.is_empty() {
            None
        } else {
            match parse_date(&due) {
                Ok(d) => Some(d),
                Err(e) => return (StatusCode::BAD_REQUEST, Json(json!({"error": e}))).into_response(),
            }
        };
    }
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

async fn api_move(State(state): State<Arc<AppState>>, Json(body): Json<MoveBody>) -> impl IntoResponse {
    let new_status = match body.status.parse::<Status>() {
        Ok(s) => s,
        Err(e) => return (StatusCode::BAD_REQUEST, Json(json!({"error": e}))).into_response(),
    };
    let mut s = state.store.lock().unwrap();
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

async fn api_add(State(state): State<Arc<AppState>>, Json(body): Json<AddBody>) -> impl IntoResponse {
    let priority = body.priority.as_deref().unwrap_or("medium");
    let priority = match priority.parse::<Priority>() {
        Ok(p) => p,
        Err(e) => return (StatusCode::BAD_REQUEST, Json(json!({"error": e}))).into_response(),
    };
    let assigned_to = body.assigned_to.unwrap_or_default();
    let tags = normalize_tags(body.tags.unwrap_or_default());
    let mut s = state.store.lock().unwrap();
    let known_ids: Vec<&str> = s.users.iter().map(|u| u.id.as_str()).collect();
    for uid in &assigned_to {
        if !known_ids.contains(&uid.as_str()) {
            return (StatusCode::BAD_REQUEST, Json(json!({"error": format!("Utilisateur inconnu: {uid}")}))).into_response();
        }
    }
    let due_date = match body.due_date.as_deref().map(parse_date).transpose() {
        Ok(d) => d,
        Err(e) => return (StatusCode::BAD_REQUEST, Json(json!({"error": e}))).into_response(),
    };
    let id = uuid::Uuid::new_v4().to_string();
    let task = crate::models::Task {
        id: id.clone(),
        title: body.title,
        priority,
        status: Status::Todo,
        assigned_to,
        tags,
        created_at: chrono::Utc::now(),
        due_date,
        is_trash: false,
    };
    s.tasks.push(task);
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"id": id})).into_response()
}

async fn api_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let store = state.store.lock().unwrap();
    Json(&store.users).into_response()
}

async fn api_comments(
    State(state): State<Arc<AppState>>,
    AxumPath(task_id): AxumPath<String>,
) -> impl IntoResponse {
    let store = state.store.lock().unwrap();
    let comments: Vec<Comment> = store
        .comments
        .iter()
        .filter(|comment| comment.task_id == task_id)
        .cloned()
        .collect();
    Json(comments).into_response()
}

async fn api_user_add(State(state): State<Arc<AppState>>, Json(body): Json<UserCreateBody>) -> impl IntoResponse {
    if body.username.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "username requis"}))).into_response();
    }
    let mut s = state.store.lock().unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    s.users.push(User {
        id: id.clone(),
        username: body.username.trim().to_string(),
        pic: body.pic.map(|s| s.to_string()),
        created_at: chrono::Utc::now(),
    });
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"id": id})).into_response()
}

async fn api_user_update(State(state): State<Arc<AppState>>, Json(body): Json<UserUpdateBody>) -> impl IntoResponse {
    if body.username.as_deref().is_some_and(|v| v.trim().is_empty()) {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "username vide"}))).into_response();
    }
    let mut s = state.store.lock().unwrap();
    let user = match s.users.iter_mut().find(|u| u.id == body.id) {
        Some(u) => u,
        None => return (StatusCode::NOT_FOUND, Json(json!({"error": format!("User ID inconnu: {}", body.id)}))).into_response(),
    };
    if body.username.is_none() && body.pic.is_none() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Spécifie au moins --username ou --pic."}))).into_response();
    }
    if let Some(name) = body.username.as_deref() {
        user.username = name.trim().to_string();
    }
    if let Some(path) = body.pic {
        user.pic = Some(path);
    }
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

async fn api_user_delete(State(state): State<Arc<AppState>>, Json(body): Json<IdBody>) -> impl IntoResponse {
    let mut s = state.store.lock().unwrap();
    if !s.users.iter().any(|u| u.id == body.id) {
        return (StatusCode::NOT_FOUND, Json(json!({"error": format!("User ID inconnu: {}", body.id)}))).into_response();
    }
    s.users.retain(|u| u.id != body.id);
    for task in s.tasks.iter_mut() {
        task.assigned_to.retain(|uid| *uid != body.id);
    }
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

async fn api_comment_add(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CommentCreateBody>,
) -> impl IntoResponse {
    if body.content.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "content requis"}))).into_response();
    }

    let mut s = state.store.lock().unwrap();
    if !s.tasks.iter().any(|task| task.id == body.task_id) {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": format!("Task ID inconnu: {}", body.task_id)}))).into_response();
    }

    if let Some(author_id) = body.author_id.as_deref() {
        if !author_id.is_empty() && !s.users.iter().any(|user| user.id == author_id) {
            return (StatusCode::BAD_REQUEST, Json(json!({"error": format!("User ID inconnu: {author_id}")}))).into_response();
        }
    }

    let comment = Comment {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: body.task_id,
        author_id: body.author_id.filter(|value| !value.trim().is_empty()),
        content: body.content.trim().to_string(),
        created_at: chrono::Utc::now(),
        updated_at: None,
    };

    s.comments.push(comment.clone());
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }

    Json(comment).into_response()
}

async fn api_task_assign(State(state): State<Arc<AppState>>, Json(body): Json<AssignBody>) -> impl IntoResponse {
    let mut s = state.store.lock().unwrap();
    let known_ids: Vec<&str> = s.users.iter().map(|u| u.id.as_str()).collect();
    let assigned_to = body.assigned_to.unwrap_or_default();
    for uid in &assigned_to {
        if !known_ids.contains(&uid.as_str()) {
            return (StatusCode::BAD_REQUEST, Json(json!({"error": format!("User ID inconnu: {uid}")}))).into_response();
        }
    }
    let task = match s.tasks.iter_mut().find(|t| t.id == body.task_id) {
        Some(t) => t,
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": format!("Task ID inconnu: {}", body.task_id)}))).into_response(),
    };
    task.assigned_to = assigned_to;
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

async fn api_del(State(state): State<Arc<AppState>>, Json(body): Json<IdBody>) -> impl IntoResponse {
    let mut s = state.store.lock().unwrap();
    let use_trash = s.config.use_trash;
    let task = match s.tasks.iter_mut().find(|t| t.id == body.id) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, Json(json!({"error": "Tâche introuvable"}))).into_response(),
    };
    if use_trash {
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

async fn api_init(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if store::is_initialized() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Déjà initialisé"}))).into_response();
    }
    let mut s = state.store.lock().unwrap();
    *s = Store::default();
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

async fn api_trash_restore(State(state): State<Arc<AppState>>, Json(body): Json<IdBody>) -> impl IntoResponse {
    let mut s = state.store.lock().unwrap();
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

async fn api_trash_clean(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut s = state.store.lock().unwrap();
    s.tasks.retain(|t| !t.is_trash);
    if let Err(e) = store::save(&s) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))).into_response();
    }
    Json(json!({"ok": true})).into_response()
}

async fn api_config(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let store = state.store.lock().unwrap();
    Json(&store.config).into_response()
}

#[derive(Deserialize)]
struct ConfigUpdateBody {
    theme_dashboard: Option<String>,
    use_trash: Option<bool>,
}

async fn api_config_update(State(state): State<Arc<AppState>>, Json(body): Json<ConfigUpdateBody>) -> impl IntoResponse {
    let mut store = state.store.lock().unwrap();
    if let Some(theme) = body.theme_dashboard {
        if theme != "dark" && theme != "light" {
            return (StatusCode::BAD_REQUEST, Json(json!({"error": "theme invalide"}))).into_response();
        }
        store.config.theme_dashboard = theme;
    }
    if let Some(use_trash) = body.use_trash {
        store.config.use_trash = use_trash;
    }
    if let Err(e) = store::save(&store) {
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

pub async fn run_server(port: u16, watch: bool) -> Result<(), String> {
    let store_data = match store::load() {
        Ok(store) => store,
        Err(e) => {
            eprintln!("Avertissement: impossible de charger kb-data.yaml: {e}");
            Store::default()
        }
    };

    let state = Arc::new(AppState {
        store: Mutex::new(store_data),
        events: broadcast::channel(32).0,
    });

    if watch {
        let watch_state = state.clone();
        tokio::spawn(async move {
            watch_files(watch_state).await;
        });
    }

    let api = Router::new()
        .route("/data", get(api_data))
        .route("/export/{format}", post(api_export))
        .route("/export/{format}/download", get(api_export_download))
        .route("/events", get(api_events))
        .route("/move", post(api_move))
        .route("/add", post(api_add))
        .route("/task-update", post(api_task_update))
        .route("/del", post(api_del))
        .route("/users", get(api_users).post(api_user_add).put(api_user_update).delete(api_user_delete))
        .route("/comments/{task_id}", get(api_comments))
        .route("/comments", post(api_comment_add))
        .route("/task-assign", post(api_task_assign))
        .route("/folder", get(api_folder))
        .route("/init", post(api_init))
        .route("/trash-restore", post(api_trash_restore))
        .route("/trash-clean", post(api_trash_clean))
        .route("/config", get(api_config).post(api_config_update))
        .with_state(state.clone());

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

fn parse_date(s: &str) -> Result<chrono::DateTime<chrono::Utc>, String> {
    let d = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| format!("Date invalide: {s}. Utilise YYYY-MM-DD."))?;
    Ok(d.and_hms_opt(0, 0, 0).unwrap().and_utc())
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
