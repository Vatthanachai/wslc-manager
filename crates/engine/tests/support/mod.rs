//! Shared helpers for tests against the real `wslc.exe` (level L6).
//!
//! Convention: `obsidians/qa/qa-integration-convention.md`. Summary:
//! - tests are `#[ignore]` and also return early unless `WSLC_INTEGRATION=1`;
//! - every resource the tests create is named `aiyara-test-<suffix>`;
//! - cleanup runs in a drop guard (also on panic) and a startup sweep removes
//!   leftovers of crashed runs, in both cases only resources with the prefix;
//! - nothing without the prefix is ever started, stopped, removed or modified.
//!
//! Nothing here runs a process unless a test calls [`wslc`] or [`wsl`], and the
//! tests that do are ignored by default.
#![allow(dead_code)] // shared by several test files; each file uses a subset

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Environment variable that opts in to the real-wslc tests.
pub const ENV_VAR: &str = "WSLC_INTEGRATION";
/// Prefix of every resource name the tests may create, touch or remove.
pub const PREFIX: &str = "aiyara-test-";
/// Reason string for `#[ignore = ...]` on every real-wslc test.
pub const IGNORE_REASON: &str = "needs real wslc.exe, run with WSLC_INTEGRATION=1 and --ignored";

/// Timeout for list and inspect calls (convention, rule 7).
pub const T_LIST: Duration = Duration::from_secs(5);
/// Timeout for start calls.
pub const T_START: Duration = Duration::from_secs(15);
/// Timeout for stop, restart and remove calls.
pub const T_STOP: Duration = Duration::from_secs(30);
/// Timeout for image pulls.
pub const T_PULL: Duration = Duration::from_secs(120);

/// True only when `WSLC_INTEGRATION=1`.
pub fn enabled() -> bool {
    std::env::var(ENV_VAR).as_deref() == Ok("1")
}

/// Runtime guard: returns `false` (and says why) when the opt-in is missing.
pub fn require_integration() -> bool {
    if enabled() {
        true
    } else {
        eprintln!("skipped: {ENV_VAR} != 1");
        false
    }
}

/// True if `name` is a resource this test suite owns.
pub fn is_test_resource(name: &str) -> bool {
    name.starts_with(PREFIX)
}

/// A fresh resource name: prefix plus a suffix unique per process and call.
pub fn unique_name(tag: &str) -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    format!("{PREFIX}{tag}-{}-{nanos:x}-{n}", std::process::id())
}

/// Result of a finished command, with everything an assertion message needs.
#[derive(Debug, Clone)]
pub struct Out {
    /// Program and arguments, for assertion messages.
    pub cmd: Vec<String>,
    /// Exit code (`-1` if the process was killed by a signal).
    pub code: i32,
    /// Raw stdout bytes.
    pub stdout: Vec<u8>,
    /// Raw stderr bytes.
    pub stderr: Vec<u8>,
}

impl Out {
    /// stdout as lossy UTF-8.
    pub fn stdout_text(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }

    /// Assertion message with command, exit code, stdout and stderr (rule 9).
    pub fn describe(&self) -> String {
        format!(
            "cmd={:?} exit={} stdout={:?} stderr={:?}",
            self.cmd,
            self.code,
            String::from_utf8_lossy(&self.stdout),
            String::from_utf8_lossy(&self.stderr)
        )
    }
}

fn drain<R: Read + Send + 'static>(pipe: Option<R>) -> std::thread::JoinHandle<Vec<u8>> {
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut p) = pipe {
            let _ = p.read_to_end(&mut buf);
        }
        buf
    })
}

/// Runs `program` with separate arguments (never a shell). A timeout is an
/// `Err`: a hang is a failure, not a wait.
pub fn run(program: &str, args: &[&str], timeout: Duration) -> Result<Out, String> {
    let cmd: Vec<String> = std::iter::once(program)
        .chain(args.iter().copied())
        .map(str::to_owned)
        .collect();
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot start {cmd:?}: {e}"))?;
    let out_reader = drain(child.stdout.take());
    let err_reader = drain(child.stderr.take());
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if start.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("timeout after {timeout:?}: {cmd:?}"));
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(e) => return Err(format!("wait failed for {cmd:?}: {e}")),
        }
    };
    Ok(Out {
        cmd,
        code: status.code().unwrap_or(-1),
        stdout: out_reader.join().unwrap_or_default(),
        stderr: err_reader.join().unwrap_or_default(),
    })
}

/// Runs `wslc.exe` with `args`.
pub fn wslc(args: &[&str], timeout: Duration) -> Result<Out, String> {
    run("wslc.exe", args, timeout)
}

/// Runs `wsl.exe` with `args` (its output is UTF-16LE, see [`decode_utf16le`]).
pub fn wsl(args: &[&str], timeout: Duration) -> Result<Out, String> {
    run("wsl.exe", args, timeout)
}

/// Decodes UTF-16LE bytes (with or without BOM), as printed by `wsl.exe`.
pub fn decode_utf16le(bytes: &[u8]) -> String {
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    String::from_utf16_lossy(&units)
        .trim_start_matches('\u{feff}')
        .to_owned()
}

/// Prints `wslc version` and `wsl --version` so the run records them (rule 8). Read-only.
pub fn print_versions() {
    match wslc(&["version"], T_LIST) {
        Ok(o) => println!("wslc: {}", o.stdout_text().trim()),
        Err(e) => println!("wslc version failed: {e}"),
    }
    match wsl(&["--version"], T_LIST) {
        Ok(o) => println!("wsl: {}", decode_utf16le(&o.stdout).trim()),
        Err(e) => println!("wsl --version failed: {e}"),
    }
}

/// Container names (the `Names` key) from `wslc container list --format json`
/// output (JSON lines), in order. Lines that are not JSON objects are skipped.
pub fn container_names(list_stdout: &str) -> Vec<String> {
    list_stdout
        .lines()
        .map(|l| l.trim().trim_start_matches('\u{feff}'))
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter_map(|v| v.get("Names").and_then(|n| n.as_str()).map(str::to_owned))
        .collect()
}

/// The subset of `names` the suite owns (prefix match only).
pub fn owned_names(names: &[String]) -> Vec<String> {
    names
        .iter()
        .filter(|n| is_test_resource(n))
        .cloned()
        .collect()
}

/// Removes one container, refusing anything without the prefix.
pub fn remove_container(name: &str) -> Result<(), String> {
    if !is_test_resource(name) {
        return Err(format!("refusing to remove non-test container {name:?}"));
    }
    wslc(&["container", "remove", "--force", name], T_STOP).map(|_| ())
}

/// Startup sweep: lists containers and removes leftovers with the prefix from
/// earlier crashed runs. Containers without the prefix are never touched.
/// Returns the removed names.
pub fn sweep() -> Result<Vec<String>, String> {
    let out = wslc(&["container", "list", "--all", "--format", "json"], T_LIST)?;
    if out.code != 0 {
        return Err(out.describe());
    }
    let leftovers = owned_names(&container_names(&out.stdout_text()));
    for name in &leftovers {
        remove_container(name)?;
    }
    Ok(leftovers)
}

/// Drop guard that removes every tracked resource, also when the test panics.
pub struct Guard {
    created: Vec<String>,
    remover: Box<dyn FnMut(&str) + Send>,
}

impl Guard {
    /// Guard that removes containers through `wslc.exe` (prefix-checked).
    pub fn new() -> Self {
        Self::with_remover(|name| {
            if let Err(e) = remove_container(name) {
                eprintln!("cleanup of {name} failed: {e}");
            }
        })
    }

    /// Guard with a custom remover (used to test the guard without a process).
    pub fn with_remover(remover: impl FnMut(&str) + Send + 'static) -> Self {
        Self {
            created: Vec::new(),
            remover: Box::new(remover),
        }
    }

    /// Tracks a resource for cleanup. Panics if the name lacks the prefix, so a
    /// test can never register (and later remove) somebody else's resource.
    pub fn track(&mut self, name: &str) {
        assert!(
            is_test_resource(name),
            "refusing to track {name:?}: name must start with {PREFIX}"
        );
        self.created.push(name.to_owned());
    }
}

impl Default for Guard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        for name in self.created.drain(..).rev() {
            (self.remover)(&name);
        }
    }
}
