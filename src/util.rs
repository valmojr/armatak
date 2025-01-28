use log::{error, info, warn};
use std::net::{IpAddr, UdpSocket};
use crate::structs::LogPayload;

pub fn log_info(data: LogPayload) -> String {
    match data.status.as_str() {
        "info" => info!("{}", data.message),
        "warn" => warn!("{}", data.message),
        "error" => error!("{}", data.message),
        _ => error!("{}","Wrong log call")
    }
    "logged".to_string()
}

pub fn get_uuid() -> String {
    use uuid::Uuid;

    let id = Uuid::new_v4().to_string();

    return id
}

pub fn get_local_address() -> String {
    fn get_local_ip() -> Result<IpAddr, String> {
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
        socket
            .connect("8.8.8.8:80")
            .map_err(|e| e.to_string())?;
        socket
            .local_addr()
            .map(|addr| addr.ip())
            .map_err(|e| e.to_string())
    }

    let parsed_data = get_local_ip();

    match parsed_data {
        Ok(ip) => {
            return format!("ws://{}:4152", ip.to_string());
        },
        Err(_) => {
            return "not provided".to_string();
        },
    }
}
