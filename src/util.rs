pub fn get_uuid() -> String {
    use uuid::Uuid;

    Uuid::new_v4().to_string()
}

mod request {
    pub fn post(data: String) -> &'static str {
        return "not implemented yet";
    }
}
