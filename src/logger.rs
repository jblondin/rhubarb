use std::fmt::Display;

pub fn log_handler_err<S: AsRef<str>, E: Display>(handler_name: S, err: E) {
    println!("Error during handler '{}': {}", handler_name.as_ref(), err);
}
