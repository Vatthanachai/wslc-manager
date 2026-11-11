use std::sync::Arc;

use aiyara_domain::{AppError, ContainerState, Scope};
use aiyara_engine::{
    parse_container_list, EngineConnector, FakeConnector, FakeRunner, ProcessOutput,
    WslcCliConnector,
};

const FIXTURE: &[u8] = include_bytes!("../../../obsidians/fixtures/container-list.json");

/// Fixtures carry a BOM and CRLF from PowerShell; strip both.
fn fixture_text() -> String {
    let text = String::from_utf8(FIXTURE.to_vec()).unwrap();
    text.trim_start_matches('\u{feff}').replace("\r\n", "\n")
}

fn scope() -> Scope {
    Scope {
        id: "wslc-cli-test".into(),
        display_name: "Test".into(),
    }
}

#[tokio::test]
async fn fake_connector_returns_one_container() {
    let c = FakeConnector;
    let scopes = c.scopes().await.unwrap();
    let list = c.list_containers(&scopes[0]).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "aiyara-spike");
    assert_eq!(list[0].state, ContainerState::Running);
}

#[test]
fn parses_fixture_json_lines() {
    let list = parse_container_list(&fixture_text(), &scope()).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, "9bf105870a7d");
    assert_eq!(list[0].image, "alpine:latest");
    assert_eq!(list[0].networks.as_deref(), Some("bridge"));
}

#[test]
fn parses_fixture_with_crlf_and_blank_lines() {
    let text = format!("\r\n{}\r\n\r\n", fixture_text().trim_end());
    assert_eq!(parse_container_list(&text, &scope()).unwrap().len(), 1);
}

#[test]
fn empty_stdout_is_empty_list() {
    assert!(parse_container_list("", &scope()).unwrap().is_empty());
    assert!(parse_container_list("\r\n", &scope()).unwrap().is_empty());
}

#[test]
fn garbage_is_parse_error_with_raw() {
    let err = parse_container_list("not json", &scope()).unwrap_err();
    assert_eq!(
        err,
        AppError::ParseError {
            raw: "not json".into()
        }
    );
}

#[test]
fn unknown_state_is_kept_raw() {
    let list = parse_container_list(r#"{"ID":"a","State":"paused"}"#, &scope()).unwrap();
    assert_eq!(list[0].state, ContainerState::Unknown("paused".into()));
}

#[tokio::test]
async fn wslc_connector_uses_injected_runner() {
    let runner = FakeRunner::with_stdout(fixture_text().into_bytes());
    let conn = WslcCliConnector::new(runner);
    let list = conn.list_containers(&scope()).await.unwrap();
    assert_eq!(list.len(), 1);
}

#[tokio::test]
async fn wslc_connector_passes_args_separately() {
    let runner = Arc::new(FakeRunner::with_stdout(Vec::new()));
    let conn = WslcCliConnector::new(runner.clone());
    assert!(conn.list_containers(&scope()).await.unwrap().is_empty());
    let calls = runner.calls();
    assert_eq!(calls[0].0, "wslc.exe");
    assert_eq!(
        calls[0].1,
        [
            "--session",
            "wslc-cli-test",
            "container",
            "list",
            "--all",
            "--format",
            "json"
        ]
    );
}

#[tokio::test]
async fn wslc_connector_maps_nonzero_exit() {
    let runner = FakeRunner::with_output(ProcessOutput {
        exit_code: 1,
        stdout: Vec::new(),
        stderr: b"boom".to_vec(),
    });
    let err = WslcCliConnector::new(runner)
        .list_containers(&scope())
        .await
        .unwrap_err();
    assert!(matches!(err, AppError::CommandFailed { .. }));
}
