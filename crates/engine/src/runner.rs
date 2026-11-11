use aiyara_domain::AppError;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Captured result of a finished process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessOutput {
    /// Exit code (0 is success).
    pub exit_code: i32,
    /// Raw stdout bytes.
    pub stdout: Vec<u8>,
    /// Raw stderr bytes.
    pub stderr: Vec<u8>,
}

/// Runs an external program with separate arguments (never through a shell).
#[async_trait]
pub trait ProcessRunner: Send + Sync {
    /// Runs `program` with `args`, failing with `AppError::Timeout` after `timeout`.
    async fn run(
        &self,
        program: &str,
        args: &[String],
        timeout: Duration,
    ) -> Result<ProcessOutput, AppError>;
}

/// Test runner returning canned output and recording the calls it received.
pub struct FakeRunner {
    output: ProcessOutput,
    calls: Mutex<Vec<(String, Vec<String>)>>,
}

impl FakeRunner {
    /// Runner that always succeeds with `stdout`.
    pub fn with_stdout(stdout: impl Into<Vec<u8>>) -> Self {
        Self::with_output(ProcessOutput {
            exit_code: 0,
            stdout: stdout.into(),
            stderr: Vec::new(),
        })
    }

    /// Runner that always returns `output`.
    pub fn with_output(output: ProcessOutput) -> Self {
        Self {
            output,
            calls: Mutex::new(Vec::new()),
        }
    }

    /// Calls received so far as `(program, args)`.
    pub fn calls(&self) -> Vec<(String, Vec<String>)> {
        self.calls.lock().map(|c| c.clone()).unwrap_or_default()
    }
}

#[async_trait]
impl ProcessRunner for FakeRunner {
    async fn run(
        &self,
        program: &str,
        args: &[String],
        _timeout: Duration,
    ) -> Result<ProcessOutput, AppError> {
        if let Ok(mut calls) = self.calls.lock() {
            calls.push((program.to_string(), args.to_vec()));
        }
        Ok(self.output.clone())
    }
}

#[async_trait]
impl<T: ProcessRunner + ?Sized> ProcessRunner for Arc<T> {
    async fn run(
        &self,
        program: &str,
        args: &[String],
        timeout: Duration,
    ) -> Result<ProcessOutput, AppError> {
        (**self).run(program, args, timeout).await
    }
}
