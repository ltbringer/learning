/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let level_string = format!("{:?}", level).to_uppercase();
    format!("[{}]: {}", level_string, message)
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
