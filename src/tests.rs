#[cfg(test)]
mod tests {
    use crate::init;
    use std::vec;
    use uuid::Uuid;

    #[test]
    fn uuid_output_is_uuid4_identifier() {
        let extension = init().testing();
        let (output, _) = extension.call("uuid", None);

        let validation = Uuid::parse_str(&output);

        assert!(validation.is_ok())
    }

    #[test]
    fn uuid_output_throws_if_passed_args() {
        let extension = init().testing();
        let args: Vec<String> = vec![1.to_string(), 2.to_string()];
        let (output, _) = extension.call("uuid", Some(args));

        assert_eq!(output, "")
    }
}
