//! Engine-neutral domain model. Independent of the UI and of any container engine.
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Container runtime a container belongs to. Only WSLC is in scope for the MVP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Runtime {
    /// WSL containers, driven through `wslc.exe`.
    Wslc,
}

/// Where containers live: a WSLC session (id plus display name).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scope {
    /// Session id as reported by the engine.
    pub id: String,
    /// Human-readable session name.
    pub display_name: String,
}

/// Container state. Unknown engine values are kept verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "raw", rename_all = "camelCase")]
pub enum ContainerState {
    /// `running`.
    Running,
    /// `exited`.
    Exited,
    /// Any other value, with the raw string.
    Unknown(String),
}

impl ContainerState {
    /// Maps the raw engine string to a state.
    ///
    /// ```
    /// use aiyara_domain::ContainerState;
    /// assert_eq!(ContainerState::from_raw("running"), ContainerState::Running);
    /// assert_eq!(ContainerState::from_raw("zzz"), ContainerState::Unknown("zzz".into()));
    /// ```
    pub fn from_raw(raw: &str) -> Self {
        match raw {
            "running" => Self::Running,
            "exited" => Self::Exited,
            other => Self::Unknown(other.to_string()),
        }
    }
}

/// Engine-neutral container. Display fields stay as the strings the engine printed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Container {
    /// Runtime.
    pub runtime: Runtime,
    /// Scope (session).
    pub scope: Scope,
    /// Container id (12 chars in list output; match by prefix).
    pub id: String,
    /// Container name.
    pub name: String,
    /// Image reference.
    pub image: String,
    /// Parsed state.
    pub state: ContainerState,
    /// Display-only status text, never parsed for logic.
    pub status_text: Option<String>,
    /// Raw creation time string (typed parsing is a later ticket).
    pub created_at: Option<String>,
    /// Ports as printed by the engine.
    pub ports: Option<String>,
    /// Networks as printed by the engine.
    pub networks: Option<String>,
}

/// Serializable application error; `kind` is a stable code, UI text lives in the frontend.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AppError {
    /// Engine output could not be parsed; raw output attached.
    #[error("parse error")]
    ParseError {
        /// Raw output that failed to parse.
        raw: String,
    },
    /// The command exited with a failure.
    #[error("command failed")]
    CommandFailed {
        /// Engine error token, when present.
        code: Option<String>,
        /// Captured stderr.
        stderr: String,
    },
    /// The command exceeded its timeout.
    #[error("timeout")]
    Timeout,
    /// Bug or unexpected internal condition.
    #[error("internal error: {message}")]
    Internal {
        /// Diagnostic detail.
        message: String,
    },
}
