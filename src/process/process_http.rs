use axum::{
    Router,
    extract::{Path as AxumPath, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{fs, io, net::TcpListener};
use tower_http::services::ServeDir;
use tracing::warn;

#[derive(Debug, Clone)]
struct AppState {
    path: Arc<PathBuf>,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> io::Result<()> {
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    println!("Http listen: {}", addr);

    let app_state = AppState {
        path: Arc::new(path.clone()),
    };
    let router = Router::new()
        .route("/{*path}", get(handle_assets))
        .nest_service("/assets", ServeDir::new(path))
        .with_state(app_state);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

#[allow(dead_code)]
async fn handle_assets(
    State(state): State<AppState>,
    AxumPath(path): AxumPath<String>,
) -> impl IntoResponse {
    let file_path = state.path.join(&path);
    if file_path.exists() {
        if !file_path.is_dir() {
            // FIXME: assume file content is utf8 encode
            read_file_to_response(file_path).await.into_response()
        } else {
            let index_html = file_path.join("index.html");
            if index_html.exists() {
                read_file_to_response(index_html).await.into_response()
            } else {
                let mut content = String::new();
                walk_dir(file_path, &mut content).unwrap();
                let content = format!(
                    r#"
                <html>
                    <head>
                        <title>File serve</title>
                    </head>
                    <body>
                        <ul>
                            {content}
                        </ul>
                    </body>
                </html>
                "#
                );
                (StatusCode::OK, Html(content)).into_response()
            }
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            format!("{} not exists", file_path.display()),
        )
            .into_response()
    }
}

fn walk_dir(path: PathBuf, content: &mut String) -> std::io::Result<()> {
    if path.is_file() {
        content.push_str(&format!(
            r#"<li><a href="{}">{}</a></li>"#,
            path.display(),
            path.file_name().unwrap().to_string_lossy()
        ));
    } else {
        content.push_str(&format!(
            "<li>{}</li>",
            path.file_name().unwrap().to_string_lossy()
        ));
        content.push_str("<ul>");
        let dir = std::fs::read_dir(path).unwrap();
        for entry in dir {
            let path = entry?.path();
            if path.is_file() {
                content.push_str(&format!(
                    r#"<li><a href="{}">{}</a></li>"#,
                    path.display(),
                    path.file_name().unwrap().to_string_lossy()
                ));
            } else {
                walk_dir(path, content)?;
            }
        }

        content.push_str("</ul>");
    }

    Ok(())
}

async fn read_file_to_response(path: PathBuf) -> (StatusCode, String) {
    match fs::read_to_string(&path).await {
        Ok(content) => (StatusCode::OK, content),
        Err(e) => {
            warn!("Read file {} error: {}", path.display(), e);
            (StatusCode::OK, e.to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use axum::{
        body::to_bytes,
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    };

    use crate::process::process_http::{AppState, handle_assets};

    #[tokio::test]
    async fn test_handle_assets() {
        let state = State(AppState {
            path: Arc::new(std::path::Path::new(".").into()),
        });

        let path = Path("Cargo.toml".to_string());
        let response = handle_assets(state, path).await.into_response();
        let (parts, body) = response.into_parts();

        let content: String =
            String::from_utf8(to_bytes(body, usize::MAX).await.unwrap().to_vec()).unwrap();

        assert_eq!(parts.status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
