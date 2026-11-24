pub fn format_err(expected: &str, got: &String) -> String {
    format!("Expected {}, instead got: {}", expected, got)
}
