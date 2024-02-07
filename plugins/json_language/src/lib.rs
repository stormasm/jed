use std::{fs, path::PathBuf};

use plugin::prelude::*;
use serde::Deserialize;

#[import]
fn command(string: &str) -> Option<Vec<u8>>;

const SERVER_PATH: &str = "node_modules/vscode-json-languageserver/bin/vscode-json-languageserver";

#[export]
pub fn name() -> &'static str {
    "vscode-json-languageserver"
}

#[export]
pub fn server_args() -> Vec<String> {
    vec!["--stdio".into()]
}

#[export]
pub fn fetch_latest_server_version() -> Option<String> {
    #[derive(Deserialize)]
    struct NpmInfo {
        versions: Vec<String>,
    }

    let output =
        command("npm info vscode-json-languageserver --json").expect("could not run command");
    let output = String::from_utf8(output).unwrap();

    let mut info: NpmInfo = serde_json::from_str(&output).ok()?;
    info.versions.pop()
}

#[export]
pub fn fetch_server_binary(container_dir: PathBuf, version: String) -> Result<PathBuf, String> {
    let version_dir = container_dir.join(version.as_str());
    fs::create_dir_all(&version_dir)
        .map_err(|_| "failed to create version directory".to_string())?;
    let binary_path = version_dir.join(SERVER_PATH);

    if fs::metadata(&binary_path).is_err() {
        let output = command(&format!(
            "npm install vscode-json-languageserver@{}",
            version
        ));
        let output = output.map(String::from_utf8);
        if output.is_none() {
            return Err("failed to install vscode-json-languageserver".to_string());
        }

        if let Ok(entries) = fs::read_dir(&container_dir) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.as_path() != version_dir {
                    fs::remove_dir_all(&entry_path).ok();
                }
            }
        }
    }

    Ok(binary_path)
}

#[export]
pub fn cached_server_binary(container_dir: PathBuf) -> Option<PathBuf> {
    let mut last_version_dir = None;
    let entries = fs::read_dir(&container_dir).ok()?;

    for entry in entries {
        let entry = entry.ok()?;
        if entry.file_type().ok()?.is_dir() {
            last_version_dir = Some(entry.path());
        }
    }

    let last_version_dir = last_version_dir?;
    let server_path = last_version_dir.join(SERVER_PATH);
    if server_path.exists() {
        Some(server_path)
    } else {
        println!("no binary found");
        None
    }
}

#[export]
pub fn initialization_options() -> Option<String> {
    Some("{ \"provideFormatter\": true }".to_string())
}

#[export]
pub fn language_ids() -> Vec<(String, String)> {
    vec![("JSON".into(), "jsonc".into())]
}
