// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::fmt::Write as _;
use std::fs;
use std::path::{
    Path,
    PathBuf,
};
use std::process::Command;

#[test]
fn test_readme_rust_examples_compile() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = manifest_dir.join("target/markdown-doctest");
    recreate_dir(&output_dir);

    let readmes = [
        ("readme_en", manifest_dir.join("README.md")),
        ("readme_zh_cn", manifest_dir.join("README.zh_CN.md")),
    ];

    for (name, path) in readmes {
        let snippets = extract_rust_snippets(&path);
        assert!(
            !snippets.is_empty(),
            "{} should contain Rust snippets",
            path.display()
        );
        compile_snippets(&manifest_dir, &output_dir, name, &snippets);
    }
}

fn recreate_dir(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path)
            .expect("failed to remove old markdown doctest directory");
    }
    fs::create_dir_all(path)
        .expect("failed to create markdown doctest directory");
}

fn extract_rust_snippets(path: &Path) -> Vec<String> {
    let content =
        fs::read_to_string(path).expect("failed to read markdown file");
    let mut snippets = Vec::new();
    let mut in_rust = false;
    let mut current = String::new();

    for line in content.lines() {
        if let Some(language) = line.trim_start().strip_prefix("```") {
            if in_rust {
                snippets.push(current.trim().to_owned());
                current.clear();
                in_rust = false;
                continue;
            }
            in_rust = is_rust_fence(language);
            continue;
        }

        if in_rust {
            current.push_str(line);
            current.push('\n');
        }
    }

    snippets
}

fn is_rust_fence(language: &str) -> bool {
    let tag = language
        .trim()
        .split(|ch: char| ch == ',' || ch.is_whitespace())
        .next()
        .unwrap_or_default();
    matches!(tag, "rust" | "rs")
}

fn compile_snippets(
    manifest_dir: &Path,
    output_dir: &Path,
    name: &str,
    snippets: &[String],
) {
    let crate_dir = output_dir.join(name);
    let bin_dir = crate_dir.join("src/bin");
    fs::create_dir_all(&bin_dir)
        .expect("failed to create snippet bin directory");

    let manifest = build_markdown_doctest_manifest(name, manifest_dir);
    fs::write(crate_dir.join("Cargo.toml"), manifest)
        .expect("failed to write snippet Cargo.toml");

    for (index, snippet) in snippets.iter().enumerate() {
        let source = normalize_snippet(snippet);
        fs::write(bin_dir.join(format!("snippet_{index}.rs")), source)
            .expect("failed to write snippet source");
    }

    let status = Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--bins")
        .current_dir(&crate_dir)
        .env("CARGO_TARGET_DIR", output_dir.join("target"))
        .status()
        .expect("failed to run cargo check for markdown snippets");

    assert!(
        status.success(),
        "markdown Rust snippets failed to compile for {name}"
    );
}

/// Builds the temporary manifest used for compiling README snippets.
fn build_markdown_doctest_manifest(name: &str, manifest_dir: &Path) -> String {
    let manifest_path = toml_basic_string(&manifest_dir.display().to_string());

    format!(
        r#"[package]
name = "qubit-atomic-{name}-markdown-doctest"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
qubit-atomic = {{ path = "{manifest_path}" }}
"#
    )
}

/// Escapes a value for use inside a TOML basic string.
fn toml_basic_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\u{0008}' => escaped.push_str("\\b"),
            '\t' => escaped.push_str("\\t"),
            '\n' => escaped.push_str("\\n"),
            '\u{000C}' => escaped.push_str("\\f"),
            '\r' => escaped.push_str("\\r"),
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\u{0000}'..='\u{001F}' | '\u{007F}' => {
                write!(escaped, "\\u{:04X}", ch as u32)
                    .expect("writing TOML escape to String should not fail");
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn normalize_snippet(snippet: &str) -> String {
    let allow_example_noise =
        "#![allow(dead_code, unused_imports, unused_variables)]\n";
    if snippet.contains("fn main") {
        format!("{allow_example_noise}{snippet}\n")
    } else {
        format!("{allow_example_noise}fn main() {{\n{snippet}\n}}\n")
    }
}

/// Verifies that Windows dependency paths are valid TOML basic strings.
#[test]
fn test_build_markdown_doctest_manifest_escapes_windows_dependency_path() {
    let manifest = build_markdown_doctest_manifest(
        "readme_en",
        Path::new(r"D:\a\rs-atomic\rs-atomic"),
    );

    assert!(
        manifest.contains(
            r#"qubit-atomic = { path = "D:\\a\\rs-atomic\\rs-atomic" }"#
        ),
        "Windows backslashes must be escaped in the generated TOML manifest:\n{manifest}"
    );
}
