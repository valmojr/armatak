use crate::structs::{LoginInfo, LoginPayload};
use log::error;
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
        .send();

    match response {
        Ok(result) => {
            let response_body: Result<serde_json::Value, _> =
                serde_json::from_str(&result.text().unwrap());

            match response_body {
                Ok(result) => {
                    let csrf_token = result["response"]["csrf_token"].as_str();

                    match csrf_token {
                        Some(result) => {
                            return result.to_string();
                        }
                        None => {
                            let message = "ERROR: Provided JSON doesnt match a valid CSRF Token";
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

pub(crate) mod markers {
    use log::info;

    use crate::{structs::Marker, util::post_marker};

    pub fn get(placeholder: String) -> &'static str {
        info!("{}", placeholder);

        return "not implemented yet";
    }

    pub fn post(data: Vec<Marker>) -> &'static str {
        for item in data {
            post_marker(item);
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
