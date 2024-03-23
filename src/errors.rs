pub mod error_module {
    use std::fs::read_to_string;
    use std::io;

    type SampleResult<T> = std::result::Result<T, SampleError>;

    #[derive(Debug, Clone)]
    pub struct SampleError {
        message: String,
    }

    impl std::fmt::Display for SampleError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Sample error: {}", self.message)
        }
    }

    impl std::error::Error for SampleError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }
    }

    impl std::convert::From<io::Error> for SampleError {
        fn from(value: io::Error) -> Self {
            SampleError {
                message: value.to_string(),
            }
        }
    }

    pub fn read_from_file(path: String) -> SampleResult<String> {
        match read_to_string(path) {
            Err(err) => Err(err.into()),
            Ok(content) => Ok(content),
        }
    }
}
