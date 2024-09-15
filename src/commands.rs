use crate::structs::{LoginInfo, LoginPayload};
use log::info;
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
    let response: Result<reqwest::blocking::Response, reqwest::Error> = client
        .get(&parsed_address)
        .body(request_body.to_owned())
        .send();

    let response_body = response.unwrap().text().unwrap();
    info!("{}", response_body);
    return response_body;
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
