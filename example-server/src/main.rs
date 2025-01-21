mod api;

use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use axum::{routing::get, Router};
use serde::Serialize;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::spawn;
use tokio::time::interval;
use tokio::try_join;

use api::events::{get_events, DefaultEventPublisher, EventDto, EventPublisher};
use api::state::ApiState;

#[derive(Serialize)]
struct BeepEventData {
    counter_value: u32,
}

async fn send_beep(sender: Sender<u32>) -> Result<()> {
    let mut interval = interval(Duration::from_secs(1));
    let mut counter = 0u32;
    loop {
        interval.tick().await;
        counter += 1;
        sender.send(counter).await?
    }
}

async fn pump_events(
    publisher: Arc<dyn EventPublisher + Send + Sync>,
    mut receiver: Receiver<u32>,
) -> Result<()> {
    loop {
        let counter_value = receiver.recv().await.ok_or(anyhow!("Channel closed"))?;
        let data = BeepEventData { counter_value };
        let dto = EventDto::with_json_payload("beep".to_string(), data)?;
        publisher.publish(dto);
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

    let (sender, receiver) = channel(1000);
    let beep_handle = spawn(send_beep(sender));
    let pump_handle = spawn(pump_events(publisher, receiver));
    let server_handle = spawn(run_server(state));

    println!("http://localhost:3000");

    _ = try_join!(beep_handle, pump_handle, server_handle)?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
