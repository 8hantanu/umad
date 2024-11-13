use axum::{
    response::Html,
    routing::{get, post},
    Router, extract::Form,
};
use maud::{html, DOCTYPE, Markup, PreEscaped};
use serde::Deserialize;
use tokio;

#[derive(Deserialize)]
struct MaudInput {
    code: String,
}

fn render_page(preview_content: Option<String>, error_message: Option<String>) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "UMAD - Maud Live Preview" }
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🌾</text></svg>";
                link rel="stylesheet" href="https://yree.io/mold/assets/css/main.css";
                script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js" async="" {}
                script src="https://unpkg.com/htmx.org@1.9.10" {}
                script src="https://unpkg.com/hyperscript.org@0.9.12" {}
                style { "
                    .preview-container { 
                        padding: 1rem;
                        border: 1px solid #ccc;
                        min-height: 200px;
                    }
                    .error {
                        color: red;
                        padding: 1rem;
                    }
                    .editor-container {
                        display: flex;
                        gap: 2rem;
                    }
                " }
            }
            body a="auto" {
                main class="content" aria-label="Content" {
                    div class="w" {
                        h1 { "UMAD - Maud Live Preview" }
                        div class="editor-container" {
                            div style="flex: 1" {
                                form hx-post="/preview" hx-target="#preview-area" {
                                    textarea name="code" style="width: 100%; height: 400px;" {
                                        "html! {\n    // Write your Maud markup here\n}"
                                    }
                                    button type="submit" { "Preview" }
                                }
                            }
                            div style="flex: 1" {
                                div id="preview-area" class="preview-container" {
                                    @if let Some(error) = error_message {
                                        div class="error" {
                                            (error)
                                        }
                                    }
                                    @if let Some(content) = preview_content {
                                        (PreEscaped(content))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn preview(Form(input): Form<MaudInput>) -> Html<String> {
    // Here you would actually compile and render the Maud markup
    // For now, we'll just return the input as-is
    Html(input.code)
}

async fn index() -> Markup {
    render_page(None, None)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/preview", post(preview));

    println!("Server running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
