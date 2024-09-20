use log::error;

use crate::structs::{LoginInfo, LoginPayload, Marker, MarkerPayload};

pub fn get_uuid() -> String {
    use uuid::Uuid;

    let id = Uuid::new_v4().to_string();

    return id
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
                    error!("{}", error);

                    return "ERROR: failed to parse the response body to a valid JSON".to_string();
                }
            }
        }
        Err(error) => {
            error!("{}", error);

            return "ERROR: failed to fetch the OTS API".to_string();
        }
    }
}