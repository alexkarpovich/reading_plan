mod app_state;
mod axum_app;
mod errors;
mod routing;
mod schemas;

use crate::axum_app::serve;

#[tokio::main]
async fn main() {
    serve().await;
}
