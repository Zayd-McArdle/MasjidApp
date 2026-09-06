#[macro_export]
macro_rules! assert_endpoint_json_response {
    ($expected_response:expr, $actual_response:expr) => {
        match ($expected_response, $actual_response) {
            (Ok(expected), Ok(actual)) => expected.0 == actual.0,
            (Err(expected), Err(actual)) => expected == actual,
            _ => false,
        }
    };
}
