#[cfg(test)]
mod tests {
    use crate::init;
    use uuid::Uuid;

    #[test]
    fn uuid_command_accepts_no_arguments_and_rejects_arguments() {
        let extension = init().testing();

        let (output, _) = extension.call("uuid", None);
        assert!(Uuid::parse_str(&output).is_ok());

        let args = vec!["1".to_string(), "2".to_string()];
        let (output_with_args, _) = extension.call("uuid", Some(args));
        assert_eq!(output_with_args, "");
    }
}
