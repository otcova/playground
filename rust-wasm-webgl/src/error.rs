pub fn fmt<E: std::fmt::Debug>(error: E, msg: &str) -> String {
    format!("{}. Details: {:?}", msg, error)
}
