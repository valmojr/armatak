use arma_rs::{arma, Extension, Group};

#[arma]
pub fn init() -> Extension {
    Extension::build()
        .command("uuid", get_uuid)
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

mod tests;
mod structs;

fn get_uuid() -> String {
    use uuid::Uuid;

    Uuid::new_v4().to_string()
}

mod markers {
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

mod casevac {
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
