use crate::EngineConnector;
use aiyara_domain::{AppError, Container, ContainerState, Runtime, Scope};
use async_trait::async_trait;

/// Connector returning one hard-coded container, for the walking skeleton and tests.
#[derive(Debug, Default, Clone, Copy)]
pub struct FakeConnector;

impl FakeConnector {
    fn scope() -> Scope {
        Scope {
            id: "fake-session".into(),
            display_name: "Fake session".into(),
        }
    }
}

#[async_trait]
impl EngineConnector for FakeConnector {
    async fn scopes(&self) -> Result<Vec<Scope>, AppError> {
        Ok(vec![Self::scope()])
    }

    async fn list_containers(&self, scope: &Scope) -> Result<Vec<Container>, AppError> {
        Ok(vec![Container {
            runtime: Runtime::Wslc,
            scope: scope.clone(),
            id: "9bf105870a7d".into(),
            name: "aiyara-spike".into(),
            image: "alpine:latest".into(),
            state: ContainerState::Running,
            status_text: Some("Up 3 seconds".into()),
            created_at: Some("2026-11-11 14:09:43 +0700 GMT+7".into()),
            ports: Some(String::new()),
            networks: Some("bridge".into()),
        }])
    }
}
