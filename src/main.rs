use axum::{
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::path::PathBuf;

/// 首页：列出所有 sgmodule 文件，显示为可点击链接
async fn index() -> Html<String> {
    let dir = sgmodules_dir();
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".sgmodule") {
                    files.push(name.to_string());
                }
            }
        }
    }

    files.sort();

    let links: Vec<String> = files
        .iter()
        .map(|f| format!(r#"<li><a href="/file/{}">{}</a></li>"#, f, f))
        .collect();

    Html(format!(
        r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Surge Modules</title>
<style>
  body {{ font-family: monospace; max-width: 600px; margin: 40px auto; }}
  a {{ text-decoration: none; color: #0366d6; }}
  a:hover {{ text-decoration: underline; }}
  li {{ margin: 8px 0; }}
</style>
</head><body>
<h2>Surge Modules</h2>
<ul>{}</ul>
</body></html>"#,
        links.join("\n")
    ))
}

/// 查看文件内容（纯文本）
async fn view_file(Path(filename): Path<String>) -> impl IntoResponse {
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return (StatusCode::BAD_REQUEST, "invalid filename").into_response();
    }

    let path = sgmodules_dir().join(&filename);
    if !path.exists() {
        return (StatusCode::NOT_FOUND, "file not found").into_response();
    }

    match std::fs::read_to_string(&path) {
        Ok(content) => {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/plain; charset=utf-8"),
            );
            (StatusCode::OK, headers, content).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "read file failed").into_response(),
    }
}

/// 获取 sgmodules 目录路径，优先使用 SGMODULES_DIR 环境变量
fn sgmodules_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("SGMODULES_DIR") {
        return PathBuf::from(dir);
    }

    // 默认使用当前工作目录下的 sgmodules
    PathBuf::from("sgmodules")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    let app = Router::new()
        .route("/", get(index))
        .route("/file/{filename}", get(view_file));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running at http://{}", addr);
    println!("  GET /               - 首页，列出所有模块链接");
    println!("  GET /file/{{name}}    - 查看文件内容");

    axum::serve(listener, app).await.unwrap();
}
