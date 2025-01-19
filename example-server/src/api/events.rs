use std::sync::Arc;

use axum::BoxError;
use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use futures_util::stream::Stream;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use super::state::ApiState;

#[derive(Clone, Debug)]
pub struct EventDto {
    pub name: String,
    pub payload: String,
}

impl EventDto {
    pub fn with_json_payload<T: serde::Serialize>(
        name: String,
        payload: T,
    ) -> serde_json::Result<EventDto> {
        Ok(EventDto {
            name,
            payload: serde_json::to_string(&payload)?,
        })
    }
}

pub trait EventPublisher {
    fn get_stream(&self) -> BroadcastStream<EventDto>;
    fn publish(&self, evt: EventDto);
}

pub struct DefaultEventPublisher {
    tx: Sender<EventDto>,
    _rx: Receiver<EventDto>,
}

impl DefaultEventPublisher {
    pub fn new() -> DefaultEventPublisher {
        let (tx, rx) = channel(1000);
        DefaultEventPublisher { tx, _rx: rx }
    }
}

impl EventPublisher for DefaultEventPublisher {
    fn get_stream(&self) -> BroadcastStream<EventDto> {
        BroadcastStream::from(self.tx.subscribe())
    }

    fn publish(&self, evt: EventDto) {
        // Safety: https://docs.rs/tokio/latest/tokio/sync/broadcast/error/struct.SendError.html
        // A send operation can only fail if there are no active receivers,
        // implying that the message could never be received.
        self.tx
            .send(evt)
            .expect("Will not fail because we keep one Receiver instance");
    }
}

pub async fn get_events(
    State(state): State<Arc<ApiState>>,
) -> Sse<impl Stream<Item = Result<Event, BoxError>>> {
    let stream = state.be_publisher.get_stream().map(|maybe_evt| {
        maybe_evt
            .map(|evt| Event::default().event(evt.name).data(evt.payload))
            .map_err(|err| err.into())
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[tokio::test]
async fn default_publisher_delivers_published_event_via_stream() {
    use tokio_stream::StreamExt;

    let publisher = DefaultEventPublisher::new();
    let mut stream = publisher.get_stream();

    publisher.publish(EventDto {
        name: "some_name".into(),
        payload: String::new(),
    });

    let result = stream.next().await.expect("Next OK");

    assert_eq!(result.expect("Result contains value").name, "some_name");
}
