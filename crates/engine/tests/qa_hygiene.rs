//! QA static checks for REQ-000: CSP, capabilities, string resources, no network, hygiene.
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).unwrap()
}

#[test]
fn forbid_unsafe_in_domain_engine_and_app() {
    for f in [
        "crates/domain/src/lib.rs",
        "crates/engine/src/lib.rs",
        "src-tauri/src/main.rs",
    ] {
        assert!(read(f).contains("#![forbid(unsafe_code)]"), "{f}");
    }
}

#[test]
fn csp_is_strict() {
    let conf: serde_json::Value = serde_json::from_str(&read("src-tauri/tauri.conf.json")).unwrap();
    let csp = conf["app"]["security"]["csp"].as_str().expect("csp set");
    assert!(csp.contains("default-src 'self'"));
    assert!(!csp.contains("unsafe-eval"));
    assert!(!csp.contains("unsafe-inline"));
    assert!(!csp.contains('*'));
    assert!(!csp.contains("https:"));
    for tok in csp.split_whitespace() {
        if tok.starts_with("http") {
            assert!(
                tok.starts_with("http://ipc.localhost") || tok.starts_with("http://localhost"),
                "unexpected origin {tok}"
            );
        }
    }
}

#[test]
fn capabilities_grant_no_permissions() {
    let dir = root().join("src-tauri/capabilities");
    let mut n = 0;
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        let v: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap();
        assert_eq!(v["permissions"].as_array().unwrap().len(), 0, "{p:?}");
        n += 1;
    }
    assert!(n >= 1);
}

#[test]
fn ui_text_only_in_strings_module() {
    let main = read("src/main.ts");
    for (i, line) in main.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("import") || t.starts_with("//") {
            continue;
        }
        let mut rest = t;
        while let Some(s) = rest.find('"') {
            let after = &rest[s + 1..];
            let Some(e) = after.find('"') else { break };
            let lit = &after[..e];
            assert!(
                !lit.contains(' ') && !lit.ends_with('.'),
                "line {}: possible hard-coded UI text {lit:?}",
                i + 1
            );
            rest = &after[e + 1..];
        }
    }
    assert!(
        !read("index.html").contains("<title>A"),
        "title must come from strings.ts"
    );
}

#[test]
fn frontend_has_no_network_calls_or_unsafe_dom() {
    let main = read("src/main.ts");
    for bad in [
        "fetch(",
        "XMLHttpRequest",
        "WebSocket",
        "http://",
        "https://",
        "eval(",
        "innerHTML",
    ] {
        assert!(!main.contains(bad), "{bad}");
    }
}

#[test]
fn no_network_crates_in_dependencies() {
    for f in [
        "Cargo.toml",
        "crates/domain/Cargo.toml",
        "crates/engine/Cargo.toml",
        "src-tauri/Cargo.toml",
    ] {
        let t = read(f);
        for bad in [
            "reqwest",
            "hyper",
            "ureq",
            "tauri-plugin-http",
            "tauri-plugin-shell",
        ] {
            assert!(!t.contains(bad), "{f}: {bad}");
        }
    }
}

#[test]
fn gitignore_covers_build_output() {
    let g = read(".gitignore");
    for p in ["/target/", "/node_modules/", "/dist/"] {
        assert!(g.contains(p), "{p}");
    }
}

#[test]
fn no_personal_paths_in_sources_and_fixtures() {
    let needles = [
        "C:\\Users\\",
        "C:/Users/",
        "Z:\\source",
        "/z/source",
        "/home/",
    ];
    let files = [
        "src/main.ts",
        "src/strings.ts",
        "index.html",
        "package.json",
        "src-tauri/tauri.conf.json",
        "src-tauri/src/main.rs",
        "crates/domain/src/lib.rs",
        "crates/engine/src/lib.rs",
        "crates/engine/src/wslc.rs",
        "crates/engine/src/runner.rs",
        "crates/engine/src/fake.rs",
        "README.md",
    ];
    for f in files {
        let t = read(f);
        for n in needles {
            assert!(!t.contains(n), "{f}: {n}");
        }
    }
    let fx = root().join("obsidians/fixtures");
    for e in std::fs::read_dir(&fx).unwrap() {
        let p = e.unwrap().path();
        if p.is_file() {
            let b = std::fs::read(&p).unwrap();
            let t = String::from_utf8_lossy(&b);
            for n in needles {
                assert!(!t.contains(n), "{p:?}: {n}");
            }
        }
    }
}
