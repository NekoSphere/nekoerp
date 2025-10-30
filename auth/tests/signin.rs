use auth::{api::prelude::*, app};
use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[tokio::test]
async fn test_signin_user_not_found() {
    let usecase: UseCase<InMemoryReposiroty, InMemoryReposiroty> =
        UseCase::new().repository(UseCaseRepository(InMemoryReposiroty::new()));

    let usecase = Arc::new(usecase);

    let app = app(usecase);
    let user = UserAuth::new();
    user.print().err().await;
    let payload = json!(user);

    let request = Request::builder()
        .method("POST")
        .uri("/signin")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_signin_success() {
    let hash = Argon::new()
        .secret(ArgonSecret(
            env::var("ARGON_SECRET_KEY").expect("argon secret key must be exist"),
        ))
        .password(ArgonPassword("Exemplo@123".into()))
        .encrypt()
        .expect("argon encrypt must be success");
    let im = InMemoryReposiroty::new().data(InMemoryReposirotyData(vec![
        User::new().password(UserPassword(hash)),
    ]));
    let usecase: UseCase<InMemoryReposiroty, InMemoryReposiroty> =
        UseCase::new().repository(UseCaseRepository(im));

    let usecase = Arc::new(usecase);

    let app = app(usecase);
    let user = UserAuth::new();
    user.print().err().await;
    let payload = json!(user);

    let request = Request::builder()
        .method("POST")
        .uri("/signin")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
