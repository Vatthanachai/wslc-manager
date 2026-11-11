//! QA edge-case tests for REQ-000: parser, connector and fake runner.
use std::sync::Arc;
use std::time::Duration;

use aiyara_domain::{AppError, ContainerState, Scope};
use aiyara_engine::{
    parse_container_list, EngineConnector, FakeConnector, FakeRunner, ProcessOutput, ProcessRunner,
    WslcCliConnector,
};

fn scope() -> Scope {
    Scope {
        id: "s".into(),
        display_name: "S".into(),
    }
}

fn fixture(name: &str) -> Vec<u8> {
    let path = format!(
        "{}/../../obsidians/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read(path).unwrap()
}

#[test]
fn real_fixture_running_parsed_raw_with_bytes_as_stored() {
    // Raw bytes, no stripping by the test: whatever the repo stores must parse.
    let text = String::from_utf8(fixture("container-list.json")).unwrap();
    let list = parse_container_list(&text, &scope()).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].state, ContainerState::Running);
    assert_eq!(list[0].name, "aiyara-spike");
}

#[test]
fn real_fixture_exited_with_ports_labels() {
    let text = String::from_utf8(fixture("container-list-exited-ports.json")).unwrap();
    let list = parse_container_list(&text, &scope()).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].state, ContainerState::Exited);
    assert_eq!(list[0].id, "61aa5e7a547c");
    assert_eq!(list[0].name, "aiyara-spike2");
    assert_eq!(
        list[0].status_text.as_deref(),
        Some("Exited (3) 3 seconds ago")
    );
    assert_eq!(list[0].networks.as_deref(), Some("bridge"));
}

#[test]
fn both_fixtures_concatenated_give_two_rows() {
    let mut text = String::from_utf8(fixture("container-list.json")).unwrap();
    text = text.trim_start_matches('\u{feff}').to_string();
    text.push_str("\r\n");
    text.push_str(&String::from_utf8(fixture("container-list-exited-ports.json")).unwrap());
    assert_eq!(parse_container_list(&text, &scope()).unwrap().len(), 2);
}

#[test]
#[ignore = "BUG-001 parse_container_list rejects a leading UTF-8 BOM (U+FEFF is not trimmed)"]
fn utf8_bom_prefixed_output_is_accepted() {
    // The fixtures README says files carry a BOM; the dev claims BOM is handled.
    let text = "\u{feff}{\"ID\":\"a\",\"Names\":\"n\",\"State\":\"running\"}\r\n";
    let list = parse_container_list(text, &scope()).unwrap();
    assert_eq!(list.len(), 1);
}

#[test]
#[ignore = "BUG-001 parse_container_list rejects a BOM-only output"]
fn bom_only_is_empty_list() {
    assert!(parse_container_list("\u{feff}", &scope())
        .unwrap()
        .is_empty());
}

#[test]
fn trailing_line_without_newline() {
    let list = parse_container_list("{\"ID\":\"a\"}\n{\"ID\":\"b\"}", &scope()).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[1].id, "b");
}

#[test]
fn json_array_input_is_parse_error_not_panic() {
    let err = parse_container_list("[{\"ID\":\"a\"}]", &scope()).unwrap_err();
    assert!(matches!(err, AppError::ParseError { .. }));
}

#[test]
fn one_bad_line_fails_whole_list_and_keeps_full_raw() {
    let raw = "{\"ID\":\"a\"}\ngarbage\n{\"ID\":\"b\"}";
    match parse_container_list(raw, &scope()).unwrap_err() {
        AppError::ParseError { raw: r } => assert_eq!(r, raw),
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn json_scalar_and_null_lines_are_parse_errors() {
    for bad in ["42", "null", "\"str\"", "true"] {
        assert!(
            matches!(
                parse_container_list(bad, &scope()),
                Err(AppError::ParseError { .. })
            ),
            "{bad}"
        );
    }
}

#[test]
fn wrong_typed_value_is_parse_error() {
    let r = parse_container_list("{\"ID\":12345}", &scope());
    assert!(matches!(r, Err(AppError::ParseError { .. })));
}

#[test]
fn null_values_become_none_or_default() {
    let list = parse_container_list(
        "{\"ID\":null,\"Names\":null,\"State\":null,\"Status\":null,\"Ports\":null}",
        &scope(),
    )
    .unwrap();
    assert_eq!(list[0].status_text, None);
    assert_eq!(list[0].ports, None);
    assert_eq!(list[0].state, ContainerState::Unknown(String::new()));
}

#[test]
fn empty_object_gives_defaults() {
    let list = parse_container_list("{}", &scope()).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, "");
    assert_eq!(list[0].created_at, None);
    assert_eq!(list[0].networks, None);
}

#[test]
fn duplicate_keys_do_not_panic() {
    // Either outcome is acceptable; no panic.
    let r = parse_container_list("{\"ID\":\"a\",\"ID\":\"b\"}", &scope());
    match r {
        Ok(l) => assert_eq!(l.len(), 1),
        Err(e) => assert!(matches!(e, AppError::ParseError { .. })),
    }
}

#[test]
fn state_matching_is_case_sensitive_and_raw() {
    let list = parse_container_list("{\"State\":\"Running\"}", &scope()).unwrap();
    assert_eq!(list[0].state, ContainerState::Unknown("Running".into()));
    let list = parse_container_list("{\"State\":\"exited\"}", &scope()).unwrap();
    assert_eq!(list[0].state, ContainerState::Exited);
}

#[test]
fn non_ascii_and_very_long_names() {
    let long = "x".repeat(1_000_000);
    let line = format!("{{\"ID\":\"a\",\"Names\":\"{long}\",\"Image\":\"café-日本語:1\"}}");
    let list = parse_container_list(&line, &scope()).unwrap();
    assert_eq!(list[0].name.len(), 1_000_000);
    assert_eq!(list[0].image, "café-日本語:1");
}

#[test]
fn huge_output_many_rows() {
    let mut s = String::new();
    for i in 0..20_000 {
        s.push_str(&format!("{{\"ID\":\"{i}\",\"State\":\"running\"}}\r\n"));
    }
    assert_eq!(parse_container_list(&s, &scope()).unwrap().len(), 20_000);
}

#[test]
fn unicode_escapes_in_json_are_decoded() {
    let list = parse_container_list("{\"Names\":\"\\u65e5\\u672c\"}", &scope()).unwrap();
    assert_eq!(list[0].name, "日本");
}

#[test]
fn scope_is_copied_into_each_container() {
    let sc = Scope {
        id: "wslc-cli-x".into(),
        display_name: "Disp".into(),
    };
    let list = parse_container_list("{}\n{}", &sc).unwrap();
    assert!(list.iter().all(|c| c.scope == sc));
}

#[tokio::test]
async fn connector_non_utf8_stdout_does_not_panic() {
    let mut bytes = b"{\"ID\":\"a\",\"Names\":\"bad".to_vec();
    bytes.push(0xff);
    bytes.extend_from_slice(b"\"}\n");
    let conn = WslcCliConnector::new(FakeRunner::with_stdout(bytes));
    let list = conn.list_containers(&scope()).await.unwrap();
    assert_eq!(list.len(), 1);
}

#[tokio::test]
async fn connector_utf16le_stdout_is_parse_error_not_panic() {
    let utf16: Vec<u8> = "{\"ID\":\"a\"}\n"
        .encode_utf16()
        .flat_map(|u| u.to_le_bytes())
        .collect();
    let conn = WslcCliConnector::new(FakeRunner::with_stdout(utf16));
    let r = conn.list_containers(&scope()).await;
    assert!(matches!(r, Err(AppError::ParseError { .. })));
}

#[tokio::test]
async fn session_names_with_spaces_and_quotes_are_single_args() {
    let ids = [
        "my session",
        "a\"b",
        "it's; rm -rf /",
        "日本 語",
        "",
        "--format",
    ];
    for id in ids {
        let runner = Arc::new(FakeRunner::with_stdout(Vec::new()));
        let conn = WslcCliConnector::new(runner.clone());
        let sc = Scope {
            id: id.into(),
            display_name: "x".into(),
        };
        conn.list_containers(&sc).await.unwrap();
        let (prog, args) = runner.calls().remove(0);
        assert_eq!(prog, "wslc.exe");
        assert_eq!(args.len(), 7, "{id:?}");
        assert_eq!(args[0], "--session");
        assert_eq!(args[1], id);
        assert_eq!(
            &args[2..],
            ["container", "list", "--all", "--format", "json"]
        );
    }
}

#[tokio::test]
async fn very_long_session_name_passed_intact() {
    let id = "s".repeat(100_000);
    let runner = Arc::new(FakeRunner::with_stdout(Vec::new()));
    let conn = WslcCliConnector::new(runner.clone());
    let sc = Scope {
        id: id.clone(),
        display_name: "x".into(),
    };
    conn.list_containers(&sc).await.unwrap();
    assert_eq!(runner.calls()[0].1[1], id);
}

#[tokio::test]
async fn nonzero_exit_carries_stderr_and_ignores_stdout() {
    let runner = FakeRunner::with_output(ProcessOutput {
        exit_code: 1,
        stdout: b"{\"ID\":\"a\"}".to_vec(),
        stderr: b"Error code: WSLC_E_SESSION_NOT_FOUND".to_vec(),
    });
    let err = WslcCliConnector::new(runner)
        .list_containers(&scope())
        .await
        .unwrap_err();
    match err {
        AppError::CommandFailed { stderr, .. } => {
            assert!(stderr.contains("WSLC_E_SESSION_NOT_FOUND"))
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[tokio::test]
async fn negative_exit_code_is_failure() {
    let runner = FakeRunner::with_output(ProcessOutput {
        exit_code: -1,
        stdout: Vec::new(),
        stderr: Vec::new(),
    });
    let r = WslcCliConnector::new(runner)
        .list_containers(&scope())
        .await;
    assert!(matches!(r, Err(AppError::CommandFailed { .. })));
}

#[tokio::test]
async fn runner_error_such_as_timeout_propagates() {
    struct Timing;
    #[async_trait::async_trait]
    impl ProcessRunner for Timing {
        async fn run(&self, _: &str, _: &[String], _: Duration) -> Result<ProcessOutput, AppError> {
            Err(AppError::Timeout)
        }
    }
    let r = WslcCliConnector::new(Timing)
        .list_containers(&scope())
        .await;
    assert_eq!(r.unwrap_err(), AppError::Timeout);
}

#[tokio::test]
async fn wslc_scopes_is_internal_error_for_now() {
    let r = WslcCliConnector::new(FakeRunner::with_stdout(Vec::new()))
        .scopes()
        .await;
    assert!(matches!(r, Err(AppError::Internal { .. })));
}

#[tokio::test]
async fn fake_connector_is_object_safe_and_echoes_scope() {
    let c: Arc<dyn EngineConnector> = Arc::new(FakeConnector);
    let sc = Scope {
        id: "other".into(),
        display_name: "Other".into(),
    };
    let list = c.list_containers(&sc).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].scope, sc);
    assert_eq!(c.scopes().await.unwrap().len(), 1);
}

#[tokio::test]
async fn app_error_serializes_with_stable_kind_codes() {
    let cases = [
        (AppError::Timeout, "timeout"),
        (AppError::ParseError { raw: "x".into() }, "parseError"),
        (
            AppError::CommandFailed {
                code: None,
                stderr: String::new(),
            },
            "commandFailed",
        ),
        (
            AppError::Internal {
                message: "m".into(),
            },
            "internal",
        ),
    ];
    for (e, kind) in cases {
        let v = serde_json::to_value(&e).unwrap();
        assert_eq!(v["kind"], kind);
    }
}

#[test]
fn container_state_serializes_as_frontend_expects() {
    let v = serde_json::to_value(ContainerState::Running).unwrap();
    assert_eq!(v["kind"], "running");
    let v = serde_json::to_value(ContainerState::Unknown("paused".into())).unwrap();
    assert_eq!(v["kind"], "unknown");
    assert_eq!(v["raw"], "paused");
}
