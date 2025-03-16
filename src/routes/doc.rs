use std::fs;
use axum::http::{StatusCode};
use axum::response::{Html, IntoResponse};

pub async fn openapi_yaml() -> impl IntoResponse {
    match fs::read_to_string("./openapi.yaml") {
        Ok(content) => {
            (StatusCode::OK, [("Content-Type", "text/yaml")], content).into_response()
        },
        Err(_) => {
            (StatusCode::NOT_FOUND, "OpenAPI specification not found").into_response()
        }
    }
}

pub async fn doc() -> impl IntoResponse {
    let html = r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>API Documentation</title>
        <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui.css" />
    </head>
    <body>
        <div id="swagger-ui"></div>
        <script src="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui-bundle.js" crossorigin></script>
        <script>
            window.onload = () => {
                window.ui = SwaggerUIBundle({
                    url: '/openapi.yaml',
                    dom_id: '#swagger-ui',
                    presets: [
                        SwaggerUIBundle.presets.apis,
                        SwaggerUIBundle.SwaggerUIStandalonePreset
                    ],
                    layout: "BaseLayout",
                });
            };
        </script>
    </body>
    </html>"#;

    Html(html).into_response()
}