use std::time::Instant;

use anyhow::Context;
use axum::{
    extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse, routing::get,
    Json, Router,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use serde::{Deserialize, Serialize};

const EXPONENTIAL_SECONDS: &[f64] = &[
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];

pub fn observability_router() -> Router {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> Json<PingResponse> {
    Json(PingResponse::default())
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct PingResponse {
    pub message: String,
}

impl PingResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Default for PingResponse {
    fn default() -> Self {
        Self::new(String::from("API is responsive"))
    }
}

pub async fn track_metrics<B>(request: Request<B>, next: Next<B>) -> impl IntoResponse {
    let path = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        request.uri().path().to_owned()
    };

    let start = Instant::now();
    let method = request.method().clone();
    let response = next.run(request).await;
    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}

pub fn recorder_handle() -> anyhow::Result<PrometheusHandle> {
    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full(String::from("http_requests_duration_seconds")),
            EXPONENTIAL_SECONDS,
        )
        .context("could not setup buckets for metrics, verify matchers are correct")?
        .install_recorder()
        .context("could not install metrics recorder")
}
