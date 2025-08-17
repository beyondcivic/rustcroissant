use std::env;
use std::fs;
use std::process::Command;

fn main() {
    // Only rerun if build script or Cargo.toml changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Get version from Cargo.toml
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "dev".to_string());

    // Get git hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "dev".to_string());

    // Generate build time in RFC3339 format
    let build_time = chrono::Utc::now().to_rfc3339();

    // Create output file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = format!("{}/version_info.rs", out_dir);

    let content = format!(
        r#"
pub static APP_NAME: &str = "rustcroissant";
pub static VERSION: &str = "{}";
pub static GIT_HASH: &str = "{}";
pub static BUILD_TIME: &str = "{}";
        "#,
        version, git_hash, build_time
    );

    fs::write(dest_path, content).unwrap();
}
