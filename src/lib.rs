use axum::{response::Html, routing::get, Router};
use once_cell::sync::Lazy;
use pulldown_cmark::{html, Parser};
use std::sync::Mutex;

static CONTENT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn write(md: &str) {
    let parser = Parser::new(md);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    {
        let mut content = CONTENT.lock().unwrap();
        *content = html_output;
    }

    // Launch runtime internally
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let app = Router::new().route("/", get(handler));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();

        println!("Running at http://localhost:3000");

        axum::serve(listener, app).await.unwrap();
    });
}

async fn handler() -> Html<String> {
    let content = CONTENT.lock().unwrap().clone();

    Html(format!(
        r#"
        <html>
            <head>
                <meta charset="utf-8">
                <style>
                    body {{ font-family: sans-serif; padding: 40px; }}
                </style>
            </head>
            <body>
                {}
            </body>
        </html>
        "#,
        content
    ))
}