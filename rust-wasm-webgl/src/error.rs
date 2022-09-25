use crate::console;

pub fn fmt<E: std::fmt::Debug>(error: E, msg: &str) -> String {
    format!("{}. Details: {:?}", msg, error)
}

pub fn report<T>(result: Result<T, String>) {
    if let Err(error) = result {
        console::log!(" ----- [ERROR] -----\n{}", error);
    }
}