use log::{error, info, warn};
use reqwest::Client;
use std::net::{IpAddr, UdpSocket};
use crate::structs::{LogPayload, LoginInfo, LoginPayload, Marker, MarkerPayload};

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

pub fn parse_login_to_payload(login_payload: LoginPayload) -> LoginInfo {
    return LoginInfo {
        username: login_payload.username.to_owned(),
        password: login_payload.password.to_owned()
    }
}

pub fn parse_marker_to_payload(marker: Marker) -> MarkerPayload {
    return MarkerPayload {
        uid: marker.uid,
        longitude: marker.longitude,
        latitude: marker.latitude,
        name: marker.name,
        r#type: marker.r#type,
        course: marker.course,
        speed: marker.speed,
        hae: marker.hae
    }
}

pub async fn async_post_markers(data: Vec<Marker>) {
    let client = Client::new();

    let authentication_token = data[0].api_auth_token.clone();
    let parsed_address: String =
        data[0].api_address.clone() + "/api/markers?auth_token=" + &authentication_token;

    for marker in data {
        let payload = parse_marker_to_payload(marker);
        let request_body = serde_json::to_string(&payload).unwrap();

        let response = client
            .post(&parsed_address)
            .body(request_body)
            .header("Content-Type", "application/json")
            .send()
            .await;

        match response {
            Ok(result) => {
                info!("Received: {}", result.text().await.unwrap());
            }
            Err(error) => {
                error!("Error: {}", error)
            }
        }
    }
}

pub fn blocking_fetch_auth_token(payload: LoginInfo, api_address: String) -> String {
    let parsed_address = api_address + "/api/login?include_auth_token";

    let request_body = serde_json::to_string(&payload).unwrap();
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&parsed_address)
        .body(request_body)
        .header("Content-Type", "application/json")
        .send();

    match response {
        Ok(result) => {
            let response_body: Result<serde_json::Value, _> =
                serde_json::from_str(&result.text().unwrap());

            match response_body {
                Ok(result) => {
                    let csrf_token = result["response"]["user"]["authentication_token"].as_str();
                    info!("Provided JSON: {:?}", result.as_str());
                    match csrf_token {
                        Some(result) => {
                            return result.to_string();
                        }
                        None => {
                            let message = "ERROR: Provided JSON doesnt match a valid Authentication Token";
                            error!("{}", message);

                            return message.to_string();
                        }
                    }
                }
                Err(error) => {
                    error!("ERROR: failed to parse the response body to a valid JSON: {}", error);

                    return "ERROR: failed to parse the response body to a valid JSON".to_string();
                }
            }
        }
        Err(error) => {
            error!("ERROR: failed to fetch the OTS API: {}", error);

            return "ERROR: failed to fetch the OTS API".to_string();
        }
    }
}