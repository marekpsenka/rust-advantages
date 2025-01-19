use super::events::EventPublisher;
use std::sync::Arc;

/// Represents state shared by handlers of the API.
pub struct ApiState {
    pub be_publisher: Arc<dyn EventPublisher + Send + Sync>,
}

impl ApiState {
    pub fn new(be_publisher: Arc<dyn EventPublisher + Send + Sync>) -> Self {
        Self { be_publisher }
    }
}
