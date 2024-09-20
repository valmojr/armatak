use crate::{
    structs::LoginPayload,
    util::{blocking_fetch_auth_token, parse_login_to_payload},
};

pub fn get_auth_token(login_payload: LoginPayload) -> String {
    let api_address = login_payload.address.clone();
    let login_info = parse_login_to_payload(login_payload);

    return blocking_fetch_auth_token(login_info, api_address);
}

pub(crate) mod markers {
    use log::{error, info};

    use crate::{
        structs::Marker,
        util::parse_marker_to_payload,
    };

    pub fn get(placeholder: String) -> &'static str {
        info!("{}", placeholder);

        return "not implemented yet";
    }

    pub fn post(data: Vec<Marker>) -> &'static str {
        for item in data {
            info!("{} - {}", item.uid, item.name);
        }

        return "not implemented yet";
    }

    pub fn post_debug(data: Vec<Marker>) -> String {
        let client = reqwest::blocking::Client::new();

        let authentication_token = data[0].api_auth_token.clone();
        let parsed_address: String = data[0].api_address.clone() + "/api/markers?auth_token=" + &authentication_token;

        let mut status: String = "fetching".to_string();

        info!("{}", status);

        for marker in data {
            let payload = parse_marker_to_payload(marker);
            let request_body = serde_json::to_string(&payload).unwrap();

            info!(
                "Parsing: {}, to {} with {}",
                request_body, parsed_address, authentication_token
            );

            let response = client
                .post(parsed_address)
                .body(request_body)
                .header("Content-Type", "application/json")
                .send();

            match response {
                Ok(result) => {
                    status = result.status().to_string();
                    info!("Received: {}", result.text().unwrap());
                }
                Err(error) => {
                    status = "fetch failed".to_string();
                    error!("Error: {}", error)
                }
            }

            return status;
        }

        return "ok".to_string();
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
