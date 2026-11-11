//! Real-`wslc.exe` integration tests (level L6). Opt-in, see
//! `obsidians/qa/qa-integration-convention.md` and the README ("Integration tests").
//!
//! Run (local, manual only, never in CI):
//! `WSLC_INTEGRATION=1 cargo test -p aiyara-engine --test wslc_integration -- --ignored --test-threads=1`
//!
//! Without `--ignored` the `#[ignore]` tests are only listed as ignored and no
//! `wslc.exe` process is started. The non-ignored tests below test the helper
//! itself with no process at all.

mod support;

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

use aiyara_domain::Scope;
use aiyara_engine::parse_container_list;
use support::*;

/// Read-only example: prints the versions and lists containers through the real
/// `wslc.exe`. It creates, starts, stops and removes nothing, so it needs no
/// guard and no sweep. A test that creates resources must follow this shape:
///
/// ```text
/// if !require_integration() { return; }
/// sweep().unwrap();                  // leftovers with the prefix only
/// print_versions();
/// let name = unique_name("list");
/// let mut guard = Guard::new();      // removes on success, failure and panic
/// guard.track(&name);                // panics if the prefix is missing
/// // create `name` via wslc, assert on exit code and WSLC_E_* tokens
/// ```
#[test]
#[ignore = "needs real wslc.exe, run with WSLC_INTEGRATION=1 and --ignored"]
fn list_is_read_only_and_parses() {
    if !require_integration() {
        return;
    }
    print_versions();
    let out = wslc(&["container", "list", "--all", "--format", "json"], T_LIST)
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(out.code, 0, "{}", out.describe());
    let scope = Scope {
        id: "wslc-integration".into(),
        display_name: "Integration".into(),
    };
    let parsed = parse_container_list(&out.stdout_text(), &scope);
    assert!(parsed.is_ok(), "{} parse={parsed:?}", out.describe());
}

// ---- tests of the helper itself: no process, run by default ----

#[test]
fn only_prefixed_names_are_owned() {
    let names: Vec<String> = [
        "aiyara-test-a",
        "aiyara-test-",
        "aiyara-spike",
        "aiyara-test",
        "my-aiyara-test-x",
        "AIYARA-TEST-x",
        "",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(owned_names(&names), vec!["aiyara-test-a", "aiyara-test-"]);
}

#[test]
fn container_names_reads_json_lines_and_skips_noise() {
    let text = "\u{feff}{\"Names\":\"aiyara-test-1\"}\r\n\r\nnot json\r\n{\"Names\":\"other\"}\r\n{\"ID\":\"x\"}\r\n";
    assert_eq!(container_names(text), vec!["aiyara-test-1", "other"]);
}

#[test]
fn unique_names_have_prefix_and_differ() {
    let (a, b) = (unique_name("x"), unique_name("x"));
    assert!(is_test_resource(&a) && is_test_resource(&b));
    assert_ne!(a, b);
}

#[test]
fn guard_removes_tracked_resources_in_reverse_order() {
    let removed = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&removed);
    {
        let mut g = Guard::with_remover(move |n| sink.lock().unwrap().push(n.to_owned()));
        g.track("aiyara-test-1");
        g.track("aiyara-test-2");
    }
    assert_eq!(
        *removed.lock().unwrap(),
        vec!["aiyara-test-2", "aiyara-test-1"]
    );
}

#[test]
fn guard_cleans_up_when_the_test_panics() {
    let removed = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&removed);
    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut g = Guard::with_remover(move |n| sink.lock().unwrap().push(n.to_owned()));
        g.track("aiyara-test-1");
        panic!("assertion failed in the test body");
    }));
    assert!(result.is_err());
    assert_eq!(*removed.lock().unwrap(), vec!["aiyara-test-1"]);
}

#[test]
fn guard_refuses_to_track_a_resource_without_the_prefix() {
    let removed = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&removed);
    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut g = Guard::with_remover(move |n| sink.lock().unwrap().push(n.to_owned()));
        g.track("aiyara-spike");
    }));
    assert!(result.is_err());
    assert!(removed.lock().unwrap().is_empty());
}

#[test]
fn remove_container_refuses_names_without_the_prefix_before_any_process() {
    // Returns Err without spawning anything (the prefix check comes first).
    assert!(remove_container("aiyara-spike").is_err());
    assert!(remove_container("").is_err());
}

#[test]
fn decodes_utf16le_with_and_without_bom() {
    let with_bom: Vec<u8> = "\u{feff}WSL 3.0.5"
        .encode_utf16()
        .flat_map(|u| u.to_le_bytes())
        .collect();
    assert_eq!(decode_utf16le(&with_bom), "WSL 3.0.5");
    let without: Vec<u8> = "ab".encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    assert_eq!(decode_utf16le(&without), "ab");
}

#[test]
fn ignore_reason_matches_the_convention() {
    assert_eq!(
        IGNORE_REASON,
        "needs real wslc.exe, run with WSLC_INTEGRATION=1 and --ignored"
    );
    // Reads the process environment only; never sets it (tests run in parallel).
    assert_eq!(enabled(), std::env::var(ENV_VAR).as_deref() == Ok("1"));
}
