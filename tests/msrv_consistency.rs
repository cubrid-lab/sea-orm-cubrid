//! Guards against the minimum supported Rust version drifting between
//! `Cargo.toml`, `clippy.toml` and `ROADMAP.md`.

use std::fs;

fn read(name: &str) -> String {
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), name);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"))
}

/// Returns the quoted value of `key = "..."` from a TOML-ish file.
fn quoted_value(text: &str, key: &str) -> String {
    text.lines()
        .filter_map(|l| l.trim().strip_prefix(key))
        .filter_map(|rest| rest.trim_start().strip_prefix('='))
        .filter_map(|v| v.trim().strip_prefix('"'))
        .filter_map(|v| v.split('"').next())
        .next()
        .unwrap_or_else(|| panic!("no `{key} = \"...\"` found"))
        .to_string()
}

#[test]
fn clippy_msrv_matches_cargo_rust_version() {
    let cargo = quoted_value(&read("Cargo.toml"), "rust-version");
    let clippy = quoted_value(&read("clippy.toml"), "msrv");
    assert_eq!(
        cargo, clippy,
        "clippy.toml `msrv` must equal Cargo.toml `rust-version`"
    );
}

#[test]
fn docs_state_cargo_rust_version() {
    let cargo = quoted_value(&read("Cargo.toml"), "rust-version");
    for (file, needle) in [
        ("ROADMAP.md", format!("Rust {cargo}+")),
        ("CONTRIBUTING.md", format!("Rust {cargo} or later")),
        ("docs/TROUBLESHOOTING.md", format!("Rust {cargo}+")),
    ] {
        assert!(
            read(file).contains(&needle),
            "{file} must contain `{needle}` (Cargo.toml rust-version = {cargo})"
        );
    }
}
