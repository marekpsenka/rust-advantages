mod api;

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use axum::{routing::get, Router};
use serde::Serialize;
use tokio::task::spawn;
use tokio::time::interval;
use tokio::try_join;

use api::events::{get_events, DefaultEventPublisher, EventDto, EventPublisher};
use api::state::ApiState;

// Split  event publisher into event source and event publisher traits

#[derive(Serialize)]
struct BeepEventData {
    counter_value: u32,
}

async fn send_beep(publisher: Arc<dyn EventPublisher + Send + Sync>) -> Result<()> {
    let mut interval = interval(Duration::from_secs(1));
    let mut counter = 0u32;
    loop {
        interval.tick().await;
        let dto = EventDto::with_json_payload(
            "beep".to_string(),
            BeepEventData {
                counter_value: counter,
            },
        )?;
        publisher.publish(dto);
        counter += 1;
    }
}

async fn run_server(state: Arc<ApiState>) -> Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/events", get(get_events))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let publisher = Arc::new(DefaultEventPublisher::new());
    let state = Arc::new(ApiState::new(publisher.clone()));

    println!("http://localhost:3000");

    let beep_handle = spawn(send_beep(publisher));
    let server_handle = spawn(run_server(state));

    _ = try_join!(beep_handle, server_handle)?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
