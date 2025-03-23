use salvo::prelude::*;
use tracing::Span;

#[handler]
pub async fn logging_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let start_time = std::time::Instant::now();
    let request_id = req
        .headers()
        .get("X-Request-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    Span::current().record("request_id", &request_id);

    ctrl.call_next(req, depot, res).await;

    let duration = start_time.elapsed();
    let status = res.status_code.unwrap_or(StatusCode::OK);
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    tracing::info!(
        method = %method,
        path = %path,
        status = %status.as_u16(),
        duration_ms = duration.as_millis(),
        request_id = %request_id,
        "Request processed"
    );
}