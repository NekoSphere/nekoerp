use std::collections::HashMap;

use auth::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request, header},
};
use tower::ServiceExt;

#[tokio::test]
async fn test_register_success() {
    let usecase: UseCase<InMemoryReposiroty, InMemoryReposiroty> =
        UseCase::new().repository(UseCaseRepository(InMemoryReposiroty::new()));

    let usecase = Arc::new(usecase);

    let app = app(usecase);
    let user = UserCreate::new();
    user.print().err().await;
    let payload = json!(user);

    let request = Request::builder()
        .method("POST")
        .uri(REGISTER_NEW_USER_PATH)
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_register_user_exists() {
    let im =
        InMemoryReposiroty::new().data(InMemoryReposirotyData::new(vec![User::new()]).unwrap());
    let usecase: UseCase<InMemoryReposiroty, InMemoryReposiroty> =
        UseCase::new().repository(UseCaseRepository(im));

    let usecase = Arc::new(usecase);

    let app = app(usecase);
    let user = UserCreate::new();
    user.print().err().await;
    let payload = json!(user);

    let request = Request::builder()
        .method("POST")
        .uri(REGISTER_NEW_USER_PATH)
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_register_first_user_success() {
    let usecase: UseCase<InMemoryReposiroty, InMemoryReposiroty> =
        UseCase::new().repository(UseCaseRepository(InMemoryReposiroty::new()));

    let usecase = Arc::new(usecase);

    let app = app(usecase);
    let user = UserCreate::new();
    user.print().err().await;
    let payload = json!(user);

    let request = Request::builder()
        .method("POST")
        .uri(REGISTER_FIRST_USER_PATH)
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = to_bytes(response.into_body(), 8 * 1024)
        .await
        .expect("limit response overflow");
    let token = String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| "<non-utf8-body>".to_string());
    println!("register raw body: {}", token);
    let request = Request::builder()
        .method("POST")
        .uri(TOKEN_PATH)
        .header(header::AUTHORIZATION, token)
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), 60 * 1024)
        .await
        .expect("limit response overflow");
    let user: UserReponse = serde_json::from_slice(&bytes).expect("invalid json response");
    println!("👤 Registered UserReponse: {user:#?}");
    let policies: HashMap<&str, Policy> = user.policies.into();
    for (
        _,
        Policy {
            read,
            write,
            delete,
        },
    ) in policies
    {
        assert!(read);
        assert!(write);
        assert!(delete);
    }
}
