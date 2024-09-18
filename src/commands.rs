use crate::structs::{LoginInfo, LoginPayload};
use reqwest;
use serde_json;

pub fn get_auth_token(payload: LoginPayload) -> String {
    let login_info = LoginInfo {
        username: payload.username,
        password: payload.password,
    };

    let parsed_address = payload.address + "/api/login";
    let request_body = serde_json::to_string(&login_info).unwrap();
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&parsed_address)
        .body(request_body)
        .header("Content-Type", "application/json")
        .send()
        .unwrap();

    let response_body: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();

    let csrf_token = response_body["response"]["csrf_token"].as_str().unwrap();

    return csrf_token.to_string()
}

pub(crate) mod markers {
    use log::info;

    use crate::structs::Marker;

    pub fn get(placeholder: String) -> &'static str {
        info!("{}", placeholder);

        return "not implemented yet";
    }

    pub fn post(data: Vec<Marker>) -> &'static str {
        for item in data {
            info!("{} - {}", item.uid, item.name)
        }

        return "not implemented yet";
    }

    pub fn delete(placeholder: String) -> &'static str {
        info!("{}", placeholder);

        return "not implemented yet";
    }
}

pub(crate) mod casevac {
    pub fn get(placeholder: String) -> String {
        format!("ERROR: Not implemented yet, {}", placeholder)
    }
    pub fn post(placeholder: String) -> String {
        format!("ERROR: Not implemented yet, {}", placeholder)
    }
    pub fn delete(placeholder: String) -> String {
        format!("ERROR: Not implemented yet, {}", placeholder)
    }
}
