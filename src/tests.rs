#[cfg(test)]
mod tests {
    mod uuid_tests {
        use uuid::Uuid;
        use crate::init;

        #[test]
        fn uuid_output_is_string() {
            let extension = init().testing();
            let (output, _) = extension.call("uuid", None);
            assert_eq!(output, output.to_string())
        }

        #[test]
        fn uuid_output_is_uuid4_identifier() {
            let extension = init().testing();
            let (output, _) = extension.call("uuid", None);

            let parsed_uuid = Uuid::parse_str(&output);
            assert!(parsed_uuid.is_ok());
            assert_eq!(parsed_uuid.unwrap().get_version(), Some(uuid::Version::Random))
        }
    }

    mod markers_tests {
        use crate::init;

        #[test]
        fn get_is_defined() {
            let extension = init().testing();
            let (output, _) = extension.call("markers:get", Some(vec!["".to_string()]));
            assert_eq!(output, "ERROR: Not implemented yet, ")
        }

        #[test]
        fn post_is_defined() {
            let extension = init().testing();
            let (output, _) = extension.call("markers:post", Some(vec!["".to_string()]));
            assert_eq!(output, "ERROR: Not implemented yet, ")
        }

        #[test]
        fn delete_is_defined() {
            let extension = init().testing();
            let (output, _) = extension.call("markers:delete", Some(vec!["".to_string()]));
            assert_eq!(output, "ERROR: Not implemented yet, ")
        }
    }
}
