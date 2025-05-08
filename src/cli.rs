// SPDX-License-Identifier: Apache-2.0

use dirs::home_dir;
use std::process::Command;

const RAMALAMA_IMAGE: &str = "quay.io/ramalama/ramalama-cli:0.8.2";

#[cfg(target_os = "windows")]
fn to_container_path(path: &str) -> String {
    let mut path = path.to_string();
    if let Some(stripped) = path.strip_prefix("\\\\?\\") {
        path = stripped.to_string();
    }
    if let Some(drive_letter) = path.chars().next() {
        if path.chars().nth(1) == Some(':') {
            return format!(
                "/mnt/{}/{}",
                drive_letter.to_ascii_lowercase(),
                path[3..].replace("\\", "/")
            );
        }
    }
    path
}

#[cfg(not(target_os = "windows"))]
fn to_container_path(path: &str) -> String {
    path.to_string()
}

fn get_ramalama_store_dir() -> String {
    let home = home_dir().expect("Could not determine home directory");
    let path = home.join(".local/share/ramalama");
    to_container_path(&path.to_string_lossy())
}

pub fn run_cli(args: Vec<String>) -> i32 {
    let store_dir = get_ramalama_store_dir();
    let home = home_dir().unwrap();
    let home_str = home.to_string_lossy().to_string();
    let container_home_mount = to_container_path(&home_str);

    let mut cmd = Command::new("podman");
    cmd.arg("run")
        .arg("--rm")
        .arg("--privileged")
        .arg("--network=host")
        .arg("--mount")
        .arg("type=bind,source=/run/podman/podman.sock,target=/run/podman/podman.sock")
        .arg("--mount")
        .arg(format!(
            "type=bind,source={},target={}",
            home_str, container_home_mount
        ))
        .arg("-e")
        .arg(format!("RAMALAMA_STORE={}", store_dir))
        .arg("-it")
        .arg(RAMALAMA_IMAGE)
        .args(&args);

    let status = cmd.status().expect("Failed to execute podman");
    status.code().unwrap_or(1)
}
