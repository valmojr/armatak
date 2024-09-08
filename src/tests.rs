#[cfg(test)]
mod tests {
    use crate::init;

    #[test]
    fn uuid_is_defined() {
        let extension = init().testing();
        let (output, _) = extension.call("uuid", None);
        assert_eq!(output, output.to_string())
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
