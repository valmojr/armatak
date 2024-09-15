pub fn get_uuid() -> String {
    use uuid::Uuid;

    Uuid::new_v4().to_string()
}

mod sync_request {
    pub fn get(address: String, token: String) -> String {
        let par = address + &token;

        return par;
    }

    pub fn post(data: String) -> &'static str {
        return "not implemented yet";
    }
}
