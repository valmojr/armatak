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
