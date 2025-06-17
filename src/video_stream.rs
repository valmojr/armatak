use arma_rs::Context;
use lazy_static::lazy_static;
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

lazy_static! {
    static ref STREAM_CTRL: Mutex<Option<Sender<()>>> = Mutex::new(None);
}

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn start_stream(
    ctx: Context,
    address: String,
    port: String,
    stream_path: String,
    username: String,
    password: String,
) -> &'static str {
    #[cfg(target_os = "linux")]
    {
        let (stop_tx, stop_rx): (Sender<()>, Receiver<()>) = mpsc::channel();
        let (status_tx, status_rx): (Sender<Result<(), String>>, Receiver<Result<(), String>>) = mpsc::channel();

        let rtsp_url = if username.is_empty() || password.is_empty() {
            format!("rtsp://{}:{}/{}", address, port, stream_path)
        } else {
            format!(
                "rtsp://{}:{}@{}:{}/{}",
                username, password, address, port, stream_path
            )
        };

        let rtsp_url_clone = rtsp_url.clone();

        thread::spawn(move || {
            let mut cmd = Command::new("ffmpeg");
            cmd.args(&[
                "-f",
                "gdigrab",
                "-i",
                "desktop",
                "-f",
                "rtsp",
                "-rtsp_transport",
                "tcp",
                &rtsp_url_clone,
            ]);

            // Try to spawn ffmpeg process
            let child_result = cmd.spawn();

            match child_result {
                Ok(mut child) => {
                    let _ = status_tx.send(Ok(()));
                    if stop_rx.recv().is_err() {}
                    let _ = child.kill();
                }
                Err(e) => {
                    let _ = status_tx.send(Err(format!("Failed to start FFmpeg: {}", e)));
                }
            }
        });

        // Save the stop channel
        match STREAM_CTRL.lock() {
            Ok(mut lock) => *lock = Some(stop_tx),
            Err(e) => {
                let _ = ctx.callback_data(
                    "VIDEO ERROR",
                    "Failed to acquire lock for stream control",
                    e.to_string(),
                );
                return "stream control lock error";
            }
        }

        // Wait up to 2 seconds to see if ffmpeg started correctly
        match status_rx.recv_timeout(Duration::from_secs(2)) {
            Ok(Ok(())) => {
                let _ = ctx.callback_null("VIDEO", "FFmpeg started successfully");
                "starting video stream"
            }
            Ok(Err(e)) => {
                let _ = ctx.callback_data("VIDEO ERROR", "FFmpeg failed to start", e);
                "ffmpeg failed to start"
            }
            Err(_) => {
                let _ = ctx.callback_null("VIDEO ERROR", "FFmpeg did not respond in time");
                "ffmpeg did not respond"
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let (stop_tx, stop_rx): (Sender<()>, Receiver<()>) = mpsc::channel();
        let (status_tx, status_rx): (Sender<Result<(), String>>, Receiver<Result<(), String>>) = mpsc::channel();

        let rtsp_url = if username.is_empty() || password.is_empty() {
            format!("rtsp://{}:{}/{}", address, port, stream_path)
        } else {
            format!(
                "rtsp://{}:{}@{}:{}/{}",
                username, password, address, port, stream_path
            )
        };

        let rtsp_url_clone = rtsp_url.clone();

        thread::spawn(move || {
            let mut cmd = Command::new("ffmpeg");
            cmd.args(&[
                "-f",
                "gdigrab",
                "-i",
                "desktop",
                "-f",
                "rtsp",
                "-rtsp_transport",
                "tcp",
                &rtsp_url_clone,
            ]);

            // Try to spawn ffmpeg process
            let child_result = cmd.creation_flags(CREATE_NO_WINDOW).spawn();

            match child_result {
                Ok(mut child) => {
                    let _ = status_tx.send(Ok(()));
                    if stop_rx.recv().is_err() {}
                    let _ = child.kill();
                }
                Err(e) => {
                    let _ = status_tx.send(Err(format!("Failed to start FFmpeg: {}", e)));
                }
            }
        });

        // Save the stop channel
        match STREAM_CTRL.lock() {
            Ok(mut lock) => *lock = Some(stop_tx),
            Err(e) => {
                let _ = ctx.callback_data(
                    "VIDEO ERROR",
                    "Failed to acquire lock for stream control",
                    e.to_string(),
                );
                return "stream control lock error";
            }
        }

        // Wait up to 2 seconds to see if ffmpeg started correctly
        match status_rx.recv_timeout(Duration::from_secs(2)) {
            Ok(Ok(())) => {
                let _ = ctx.callback_null("VIDEO", "FFmpeg started successfully");
                "starting video stream"
            }
            Ok(Err(e)) => {
                let _ = ctx.callback_data("VIDEO ERROR", "FFmpeg failed to start", e);
                "ffmpeg failed to start"
            }
            Err(_) => {
                let _ = ctx.callback_null("VIDEO ERROR", "FFmpeg did not respond in time");
                "ffmpeg did not respond"
            }
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        ctx.callback_null("VIDEO ERROR", "Screen capture is only supported on Windows");
        "unsupported platform"
    }
}

pub fn stop_stream(ctx: Context) -> &'static str {
    match STREAM_CTRL.lock() {
        Ok(mut lock) => {
            if let Some(tx) = lock.take() {
                if let Err(e) = tx.send(()) {
                    let _ = ctx.callback_data(
                        "VIDEO ERROR",
                        "Failed to send stop signal",
                        e.to_string(),
                    );
                    "error sending stop"
                } else {
                    let _ = ctx.callback_null("VIDEO", "Sent stop signal to FFmpeg");
                    "stopping video stream"
                }
            } else {
                let _ = ctx.callback_null("VIDEO ERROR", "Tried to stop a nonexistent stream");
                "no stream to stop"
            }
        }
        Err(e) => {
            let _ = ctx.callback_data(
                "VIDEO ERROR",
                "Failed to acquire lock for stop",
                e.to_string(),
            );
            "lock error"
        }
    }
}