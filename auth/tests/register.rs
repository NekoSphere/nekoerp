use auth::{api::prelude::*, app};
use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[tokio::test]
async fn test_register_success() {
    let usecase: UseCase<InMemoryReposiroty, InMemoryReposiroty> = UseCase::new()
        .cache(UseCaseCache(InMemoryReposiroty::new()))
        .repository(UseCaseRepository(InMemoryReposiroty::new()));

    let usecase = Arc::new(usecase);

    let app = app(usecase);
    let user = UserCreate::new();
    user.print().err().await;
    let payload = json!(user);

    let request = Request::builder()
        .method("POST")
        .uri("/register")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}
