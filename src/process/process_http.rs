use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    Router,
    extract::{Path as AxumPath, State},
    http::StatusCode,
    routing::get,
};
use tokio::{fs, io, net::TcpListener};
use tracing::warn;

#[derive(Debug, Clone)]
struct AppState {
    path: Arc<PathBuf>,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> io::Result<()> {
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    println!("Http listen: {}", addr);
    let app_state = AppState {
        path: Arc::new(path),
    };
    let router = Router::new()
        .route("/{*path}", get(handle_assets))
        .with_state(app_state);

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, router).await?;

    Ok(())
}

async fn handle_assets(
    State(state): State<AppState>,
    AxumPath(path): AxumPath<String>,
) -> (StatusCode, String) {
    let file_path = state.path.join(&path);
    if file_path.exists() {
        match fs::read_to_string(&file_path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => {
                warn!("Read file {} error: {}", file_path.display(), e);
                (StatusCode::OK, e.to_string())
            }
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            format!("{} not exists", file_path.display()),
        )
    }
}
