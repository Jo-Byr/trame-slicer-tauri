#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use async_std::task;
use std::io::Write;
use std::time::Duration;
use tauri_plugin_shell::{process::CommandEvent, ShellExt};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
        let splashscreen_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();

        let shell = app.shell();

        let (mut rx, _child) = shell
            .sidecar("trame")?
            .args([
                "--server",
                "--port", "0",
                "--timeout", "1",
                "--no-http",
            ])
            .spawn()?;

        tauri::async_runtime::spawn(async move {
            let mut log = std::fs::File::create(
                std::env::temp_dir().join("slicer-tauri-debug.log")
            ).unwrap();

            while let Some(event) = rx.recv().await {
                match &event {
                    CommandEvent::Stdout(bytes) => {
                        let line = String::from_utf8_lossy(bytes);
                        let _ = writeln!(log, "stdout: {}", line);
                    }
                    CommandEvent::Stderr(bytes) => {
                        let line = String::from_utf8_lossy(bytes);
                        let _ = writeln!(log, "stderr: {}", line);
                    }
                    other => {
                        let _ = writeln!(log, "{:?}", other);
                    }
                }
                let _ = log.flush();

                if let CommandEvent::Stdout(line) = &event {
                    let line = String::from_utf8_lossy(line);
                    if line.contains("tauri-server-port=") {
                        let tokens: Vec<&str> = line.split("=").collect();
                        let port = tokens[1].trim();
                        let _ = main_window.eval(&format!("window.location.replace(window.location.href + '?sessionURL=ws://localhost:{}/ws')", port));
                    }
                    if line.contains("tauri-client-ready") {
                        task::sleep(Duration::from_secs(2)).await;

                        main_window.show().unwrap();
                        splashscreen_window.close().unwrap();
                    }
                }
            }
        });
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running application");
}
