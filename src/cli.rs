// SPDX-License-Identifier: Apache-2.0

use dirs::home_dir;
use std::process::Command;

#[cfg(target_os = "linux")]
use std::{env, path::PathBuf};

#[cfg(target_os = "linux")]
use std::process::exit;

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

#[cfg(target_os = "linux")]
fn get_podman_socket() -> String {
    if let Ok(xdg_runtime_dir) = env::var("XDG_RUNTIME_DIR") {
        let socket_path = PathBuf::from(&xdg_runtime_dir).join("podman/podman.sock");

        // Check if the socket file exists
        if socket_path.exists() {
            return socket_path.to_string_lossy().to_string();
        }

        // Run systemctl to check if podman.socket is enabled
        let output = Command::new("systemctl")
            .arg("--user")
            .arg("is-enabled")
            .arg("podman.socket")
            .output()
            .expect("Failed to execute systemctl");

        // Capture the status from stdout
        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Check if the status is "enabled"
        if status == "enabled" {
            eprintln!(
                "podman.socket is enabled, but the socket file {} does not exist.\n\
                 Is the socket service running?\nTry:\n  sudo systemctl start podman.socket",
                socket_path.display()
            );
            exit(1);
        }

        // Check if the status is "disabled"
        if status == "disabled" {
            eprintln!(
                "Podman socket not found at {} and podman.socket is not enabled.\n\
                 Please enable it using:\n  systemctl --user enable --now podman.socket",
                socket_path.display()
            );
            exit(1);
        }

        // If the status is neither "enabled" nor "disabled", it's an error
        eprintln!(
            "Unexpected output from systemctl: {}\n\
             Failed to determine if podman.socket is enabled or disabled.",
            status
        );
        exit(1);
    } else {
        eprintln!("XDG_RUNTIME_DIR is not set. Cannot determine Podman socket path.");
        exit(1);
    }
}

#[cfg(target_os = "macos")]
fn get_podman_socket() -> String {
    "/run/user/501/podman/podman.sock".to_string() // use rootless podman socket from podman machine
}

#[cfg(target_os = "windows")]
fn get_podman_socket() -> String {
    "/run/user/1000/podman/podman.sock".to_string() // use rootless podman socket from podman machine
}

pub fn run_cli(args: Vec<String>) -> i32 {
    let store_dir = get_ramalama_store_dir();
    let home = home_dir().unwrap();
    let home_str = home.to_string_lossy().to_string();
    let container_home_mount = to_container_path(&home_str);
    let podman_socket = get_podman_socket();

    let mut cmd = Command::new("podman");
    cmd.arg("run")
        .arg("--rm")
        .arg("--privileged")
        .arg("--network=host")
        .arg("--mount")
        .arg(format!(
            "type=bind,source={},target=/run/podman/podman.sock",
            podman_socket
        ))
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
