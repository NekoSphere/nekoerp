use super::import::*;
use crate::{Log, LogMethod, LogStatus, LogUri};

pub async fn log_middleware(req: Request<Body>, next: Next) -> Response {
    let mut log = Log::new()
        .uri(LogUri(req.uri().clone()))
        .method(LogMethod(req.method().clone()));
    let response = next.run(req).await;
    log.mut_status(LogStatus(response.status()));

    match log.status {
        status if status.is_success() => log.print().success().await,
        status if status.is_informational() => log.print().info().await,
        status if status.is_redirection() => log.print().warning().await,
        status if status.is_client_error() => log.print().err().await,
        status if status.is_server_error() => log.print().panic().await,
        _ => log.print().rust().await,
    }

    response
}
