use crate::{EngineConnector, ProcessRunner};
use aiyara_domain::{AppError, Container, ContainerState, Runtime, Scope};
use async_trait::async_trait;
use serde::Deserialize;
use std::time::Duration;

const LIST_TIMEOUT: Duration = Duration::from_secs(5);

/// Keys of `wslc container list --format json`; unknown keys are ignored, missing ones are `None`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawContainer {
    #[serde(rename = "ID")]
    id: Option<String>,
    names: Option<String>,
    image: Option<String>,
    state: Option<String>,
    status: Option<String>,
    created_at: Option<String>,
    ports: Option<String>,
    networks: Option<String>,
}

/// Parses `container list --format json` output (JSON lines; empty output is an empty list).
///
/// ```
/// use aiyara_domain::Scope;
/// let scope = Scope { id: "s".into(), display_name: "S".into() };
/// assert!(aiyara_engine::parse_container_list("", &scope).unwrap().is_empty());
/// ```
pub fn parse_container_list(stdout: &str, scope: &Scope) -> Result<Vec<Container>, AppError> {
    stdout
        // Strip one leading UTF-8 BOM; `trim` does not remove U+FEFF.
        .strip_prefix('\u{feff}')
        .unwrap_or(stdout)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            let raw: RawContainer =
                serde_json::from_str(line).map_err(|_| AppError::ParseError {
                    raw: stdout.to_string(),
                })?;
            Ok(Container {
                runtime: Runtime::Wslc,
                scope: scope.clone(),
                id: raw.id.unwrap_or_default(),
                name: raw.names.unwrap_or_default(),
                image: raw.image.unwrap_or_default(),
                state: ContainerState::from_raw(raw.state.as_deref().unwrap_or("")),
                status_text: raw.status,
                created_at: raw.created_at,
                ports: raw.ports,
                networks: raw.networks,
            })
        })
        .collect()
}

/// Connector driving `wslc.exe` through an injected [`ProcessRunner`].
pub struct WslcCliConnector<R: ProcessRunner> {
    runner: R,
    program: String,
}

impl<R: ProcessRunner> WslcCliConnector<R> {
    /// Creates a connector that runs `wslc.exe` via `runner`.
    pub fn new(runner: R) -> Self {
        Self {
            runner,
            program: "wslc.exe".into(),
        }
    }
}

#[async_trait]
impl<R: ProcessRunner> EngineConnector for WslcCliConnector<R> {
    async fn scopes(&self) -> Result<Vec<Scope>, AppError> {
        // Session discovery belongs to REQ-001.
        Err(AppError::Internal {
            message: "session discovery not implemented".into(),
        })
    }

    async fn list_containers(&self, scope: &Scope) -> Result<Vec<Container>, AppError> {
        let args: Vec<String> = [
            "--session",
            &scope.id,
            "container",
            "list",
            "--all",
            "--format",
            "json",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let out = self.runner.run(&self.program, &args, LIST_TIMEOUT).await?;
        if out.exit_code != 0 {
            return Err(AppError::CommandFailed {
                code: None,
                stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            });
        }
        parse_container_list(&String::from_utf8_lossy(&out.stdout), scope)
    }
}
