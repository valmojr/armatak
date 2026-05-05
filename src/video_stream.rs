use arma_rs::Context;
use lazy_static::lazy_static;
use log::info;
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

fn stop_existing_stream() {
    if let Ok(mut lock) = STREAM_CTRL.lock() {
        if let Some(tx) = lock.take() {
            let _ = tx.send(());
        }
    }
}

fn spawn_ffmpeg_with_args(
    mut cmd: Command,
    stop_rx: Receiver<()>,
    status_tx: Sender<Result<(), String>>,
    description: String,
) {
    thread::spawn(move || {
        info!("Starting FFmpeg video stream: {}", description);

        #[cfg(target_os = "windows")]
        let child_result = cmd.creation_flags(CREATE_NO_WINDOW).spawn();

        #[cfg(not(target_os = "windows"))]
        let child_result = cmd.spawn();

        match child_result {
            Ok(mut child) => {
                let _ = status_tx.send(Ok(()));
                let _ = stop_rx.recv();
                info!("Stopping FFmpeg video stream: {}", description);
                let _ = child.kill();
            }
            Err(e) => {
                let _ = status_tx.send(Err(format!("Failed to start FFmpeg: {}", e)));
            }
        }
    });
}

pub fn start_stream(
    ctx: Context,
    address: String,
    port: String,
    stream_path: String,
    username: String,
    password: String,
) -> &'static str {
    stop_existing_stream();

    let (stop_tx, stop_rx) = mpsc::channel();
    let (status_tx, status_rx) = mpsc::channel();

    let rtsp_url = if username.is_empty() || password.is_empty() {
        format!("rtsp://{}:{}/{}", address, port, stream_path)
    } else {
        format!("rtsp://{}:{}@{}:{}/{}", username, password, address, port, stream_path)
    };

    let mut cmd = Command::new("ffmpeg");
    #[cfg(target_os = "windows")]
    cmd.args([
        "-f",
        "gdigrab",
        "-framerate",
        "15",
        "-i",
        "desktop",
        "-an",
        "-c:v",
        "libx264",
        "-preset",
        "ultrafast",
        "-tune",
        "zerolatency",
        "-pix_fmt",
        "yuv420p",
        "-f",
        "rtsp",
        "-rtsp_transport",
        "tcp",
        &rtsp_url,
    ]);

    #[cfg(target_os = "linux")]
    cmd.args([
        "-f",
        "x11grab",
        "-framerate",
        "15",
        "-video_size",
        "1280x720",
        "-i",
        ":0.0",
        "-an",
        "-c:v",
        "libx264",
        "-preset",
        "ultrafast",
        "-tune",
        "zerolatency",
        "-pix_fmt",
        "yuv420p",
        "-f",
        "rtsp",
        "-rtsp_transport",
        "tcp",
        &rtsp_url,
    ]);

    spawn_ffmpeg_with_args(cmd, stop_rx, status_tx, format!("RTSP {}", rtsp_url));

    if let Ok(mut lock) = STREAM_CTRL.lock() {
        *lock = Some(stop_tx);
    }

    match status_rx.recv_timeout(Duration::from_secs(2)) {
        Ok(Ok(())) => {
            let _ = ctx.callback_null("VIDEO", "FFmpeg RTSP stream started successfully");
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

pub fn start_rtp_stream(ctx: Context, address: String, port: String) -> &'static str {
    stop_existing_stream();

    let (stop_tx, stop_rx) = mpsc::channel();
    let (status_tx, status_rx) = mpsc::channel();
    let rtp_url = format!("rtp://{}:{}", address, port);

    let mut cmd = Command::new("ffmpeg");

    #[cfg(target_os = "windows")]
    cmd.args([
        "-f",
        "gdigrab",
        "-framerate",
        "15",
        "-i",
        "desktop",
        "-an",
        "-c:v",
        "libx264",
        "-preset",
        "ultrafast",
        "-tune",
        "zerolatency",
        "-pix_fmt",
        "yuv420p",
        "-g",
        "30",
        "-f",
        "rtp",
        &rtp_url,
    ]);

    #[cfg(target_os = "linux")]
    cmd.args([
        "-f",
        "x11grab",
        "-framerate",
        "15",
        "-video_size",
        "1280x720",
        "-i",
        ":0.0",
        "-an",
        "-c:v",
        "libx264",
        "-preset",
        "ultrafast",
        "-tune",
        "zerolatency",
        "-pix_fmt",
        "yuv420p",
        "-g",
        "30",
        "-f",
        "rtp",
        &rtp_url,
    ]);

    spawn_ffmpeg_with_args(cmd, stop_rx, status_tx, format!("RTP {}", rtp_url));

    if let Ok(mut lock) = STREAM_CTRL.lock() {
        *lock = Some(stop_tx);
    }

    match status_rx.recv_timeout(Duration::from_secs(2)) {
        Ok(Ok(())) => {
            info!("Started RTP video stream toward {}", rtp_url);
            let _ = ctx.callback_null("VIDEO", "FFmpeg RTP stream started successfully");
            "starting RTP video stream"
        }
        Ok(Err(e)) => {
            let _ = ctx.callback_data("VIDEO ERROR", "FFmpeg failed to start RTP stream", e);
            "ffmpeg failed to start RTP stream"
        }
        Err(_) => {
            let _ = ctx.callback_null("VIDEO ERROR", "FFmpeg RTP stream did not respond in time");
            "ffmpeg RTP stream did not respond"
        }
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
