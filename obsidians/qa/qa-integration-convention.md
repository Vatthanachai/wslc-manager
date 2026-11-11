---
tags:
  - qa
  - testing
  - wslc
status: draft-v1
updated: 2026-11-11
---

# Real wslc integration-test convention

Level L6 in [[test-strategy]]. Facts about wslc: [[ba-analysis]] section 4. These tests touch the user's real WSL, so they are opt-in and defensive. Never run them in CI (A: no wslc there).

## Rules

1. Every test has `#[ignore = "needs real wslc.exe, run with WSLC_INTEGRATION=1 and --ignored"]`, plus a runtime guard: if env var `WSLC_INTEGRATION` is not `1`, the test prints why and returns early. Run: `WSLC_INTEGRATION=1 cargo test -p <crate> -- --ignored --test-threads=1`.
2. All resources (containers, sessions, networks) are named with prefix `aiyara-test-` plus a random suffix.
3. Create resources in the default session only. A dedicated session (`--session aiyara-test-...`) is allowed only after it is verified possible and removable [U]. Until then do not create sessions.
4. Cleanup in a drop guard (runs on panic too), and a startup sweep removes leftovers with the prefix from earlier crashed runs. The sweep lists, filters by prefix, and removes only matches.
5. Never touch containers, sessions or images not created by the test. Never run `session terminate`, `image remove` or `remove --force` on anything without the prefix. Never unregister, delete or import over a real distro.
6. Images: only `alpine:latest`, already present or pulled on demand. No `docker.io` pulls of anything else.
7. Every call has a timeout (list/inspect 5 s, start 15 s, stop/restart 30 s or more, pull 120 s). A hang is a failure, not a wait.
8. Print `wslc version` and `wsl --version` (decoded UTF-16LE) at the start of each run, so the report records them.
9. On failure the assertion message includes the command, exit code, stdout and stderr.
10. Assert on exit code and `WSLC_E_*` tokens, not English text (messages can be localised [U]).
11. Cases needing destructive or unverifiable setup (wslc missing, WSL too old) stay manual checks on a VM.

## Sketch (illustration only, not a code file)

```rust
struct Guard { created: Vec<String> }
impl Drop for Guard {
    fn drop(&mut self) {
        for name in &self.created { let _ = wslc(&["container", "remove", "--force", name]); }
    }
}

fn enabled() -> bool { std::env::var("WSLC_INTEGRATION").as_deref() == Ok("1") }

#[tokio::test]
#[ignore = "needs real wslc.exe, run with WSLC_INTEGRATION=1 and --ignored"]
async fn list_shows_container_created_by_test() {
    if !enabled() { eprintln!("skipped: WSLC_INTEGRATION != 1"); return; }
    sweep_prefix("aiyara-test-").await;   // leftovers from crashed runs, prefix only
    println!("wslc: {}", wslc_version().await);
    let name = format!("aiyara-test-{}", rand_suffix());
    let mut g = Guard { created: vec![] };
    run_with_timeout(&["container", "run", "-d", "--name", &name, "alpine:latest", "sleep", "60"], 30).await;
    g.created.push(name.clone());
    let out = run_with_timeout(&["container", "list", "--all", "--format", "json"], 5).await;
    assert!(out.stdout.contains(&name),
        "cmd={:?} exit={} stdout={} stderr={}", out.cmd, out.code, out.stdout, out.stderr);
}
```

`wslc`, `sweep_prefix`, `run_with_timeout` are helpers the ticket will define; the sketch fixes the shape, not the API. The `container run -d` flags are [U] until captured.

Back to [[README|QA index]].
