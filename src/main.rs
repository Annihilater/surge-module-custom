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
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    min-height: 100vh;
    padding: 40px 20px;
  }}
  .container {{
    max-width: 800px;
    margin: 0 auto;
    background: rgba(255, 255, 255, 0.95);
    border-radius: 20px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    padding: 40px;
    backdrop-filter: blur(10px);
  }}
  h1 {{
    color: #2d3748;
    font-size: 2.5em;
    margin-bottom: 10px;
    font-weight: 700;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }}
  .subtitle {{
    color: #718096;
    font-size: 1.1em;
    margin-bottom: 30px;
    padding-bottom: 20px;
    border-bottom: 2px solid #e2e8f0;
  }}
  .count {{
    display: inline-block;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 4px 12px;
    border-radius: 20px;
    font-size: 0.9em;
    font-weight: 600;
    margin-left: 10px;
  }}
  ul {{
    list-style: none;
  }}
  li {{
    margin: 12px 0;
    transition: transform 0.2s ease;
  }}
  li:hover {{
    transform: translateX(8px);
  }}
  a {{
    display: block;
    padding: 16px 20px;
    background: white;
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    text-decoration: none;
    color: #2d3748;
    font-size: 1.05em;
    transition: all 0.3s ease;
    position: relative;
    overflow: hidden;
  }}
  a::before {{
    content: '📦';
    margin-right: 12px;
    font-size: 1.2em;
  }}
  a:hover {{
    border-color: #667eea;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    box-shadow: 0 8px 20px rgba(102, 126, 234, 0.4);
  }}
  .empty {{
    text-align: center;
    padding: 60px 20px;
    color: #a0aec0;
    font-size: 1.1em;
  }}
  .empty::before {{
    content: '📭';
    display: block;
    font-size: 4em;
    margin-bottom: 20px;
  }}
  footer {{
    margin-top: 40px;
    padding-top: 20px;
    border-top: 2px solid #e2e8f0;
    text-align: center;
    color: #a0aec0;
    font-size: 0.9em;
  }}
</style>
</head><body>
<div class="container">
  <h1>🚀 Surge Modules</h1>
  <div class="subtitle">
    自定义 Surge 模块管理中心
    <span class="count">{} 个模块</span>
  </div>
  {}
  <footer>
    Powered by Rust + Axum
  </footer>
</div>
</body></html>"#,
        files.len(),
        if files.is_empty() {{
            r#"<div class="empty">暂无模块文件<br>请将 .sgmodule 文件放入 sgmodules 目录</div>"#.to_string()
        }} else {{
            format!("<ul>{}</ul>", links.join("\n"))
        }}
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

    let addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:12345".to_string());

    let app = Router::new()
        .route("/", get(index))
        .route("/file/{filename}", get(view_file));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running at http://{}", addr);
    println!("  GET /               - 首页，列出所有模块链接");
    println!("  GET /file/{{name}}    - 查看文件内容");

    axum::serve(listener, app).await.unwrap();
}
