//! Engine-neutral connector trait, process runner abstraction and connectors.
#![forbid(unsafe_code)]

mod fake;
mod runner;
mod wslc;

pub use fake::FakeConnector;
pub use runner::{FakeRunner, ProcessOutput, ProcessRunner};
pub use wslc::{parse_container_list, WslcCliConnector};

use aiyara_domain::{AppError, Container, Scope};
use async_trait::async_trait;

/// Engine-neutral access to a container engine.
#[async_trait]
pub trait EngineConnector: Send + Sync {
    /// Scopes (sessions) known to the engine.
    async fn scopes(&self) -> Result<Vec<Scope>, AppError>;
    /// All containers (running or not) in `scope`.
    async fn list_containers(&self, scope: &Scope) -> Result<Vec<Container>, AppError>;
}
