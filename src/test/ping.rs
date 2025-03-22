#[tokio::test]
async fn test_ping_endpoint() {
    use crate::create_app;
    use axum::response::Response;
    use axum::{body::Body, http::Request, http::StatusCode};
    use tower::ServiceExt;

    let app = create_app().await;

    let response: Response = app
        .oneshot(Request::get("/ping").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert_eq!(body_str, "pong");
}


