pub fn get_uuid() -> String {
    use uuid::Uuid;

    Uuid::new_v4().to_string()
}

pub(crate) mod data_parsing {

}

mod request {
    pub fn post(data: String) -> &'static str {
        return "not implemented yet";
    }

    pub fn get(data: String) -> &'static str {
        return "not implemented yet";
    }

    pub fn delete(data: String) -> &'static str {
        return "not implemented yet";
    }
}
