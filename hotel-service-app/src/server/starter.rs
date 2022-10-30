use std::{future::ready, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    http::{HeaderValue, Method},
    middleware,
    routing::get,
    Router,
};
use common_utils::{
    handle_timeout_error,
    observability::{self, recorder_handle, track_metrics},
    HTTP_TIMEOUT,
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub async fn start() -> anyhow::Result<()> {
    let record_hdl = recorder_handle()?;

    let router = Router::new()
        .route("/metrics", get(move || ready(record_hdl.render())))
        .nest("/api", observability::observability_router())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(HTTP_TIMEOUT)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap()) //TODO to modify
                .allow_methods([Method::GET]),
        )
        .route_layer(middleware::from_fn(track_metrics));
    axum::Server::bind(&format!("0.0.0.0:{}", "8081").parse().unwrap())
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
