use arma_rs::{arma, Extension, Group};

#[arma]
fn init() -> Extension {
    Extension::build()
        .group(
            "markers",
            Group::new()
                .command("get", markers::get)
                .command("post", markers::post)
                .command("delete", markers::delete),
        )
        .group(
            "casevac",
            Group::new()
                .command("get", casevac::get)
                .command("post", casevac::post)
                .command("delete", casevac::delete),
        )
        .finish()
}

#[derive(Debug, Clone)]
pub struct Marker {
    pub longitude: f64,
    pub latitude: f64,
    pub name: String,
    pub uid: String,
    pub r#type: Option<String>,
    pub course: Option<i32>,
    pub azimuth: Option<i32>,
    pub speed: Option<i32>,
    pub battery: Option<i32>,
    pub fov: Option<i32>,
    pub ce: Option<i32>,
    pub hae: Option<i32>,
    pub le: Option<i32>,
}

mod markers {
    pub fn get(placeholder: String) -> String {
        format!("Not implemented yet, {}", placeholder)
    }
    pub fn post(placeholder: String) -> String {
        format!("Not implemented yet, {}", placeholder)
    }
    pub fn delete(placeholder: String) -> String {
        format!("Not implemented yet, {}", placeholder)
    }
}

mod casevac {
    pub fn get(placeholder: String) -> String {
        format!("Not implemented yet, {}", placeholder)
    }
    pub fn post(placeholder: String) -> String {
        format!("Not implemented yet, {}", placeholder)
    }
    pub fn delete(placeholder: String) -> String {
        format!("Not implemented yet, {}", placeholder)
    }
}
