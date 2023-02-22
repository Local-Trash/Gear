/// This is a custom logging macro for the library.
/// # Examples
/// ```rust
/// use fish::{log, Log::{LogType}};
/// log!(LogType::Debug, "The player moved");
/// ```
#[macro_export]
macro_rules! log {
    ($enum:expr, $msg:expr, $err:expr) => {
        match $enum {
            #[cfg(feature = "debug")]
            LogType::Debug => println!("[{} {}] {:?}", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(), $enum.bright_yellow(), format!("{} {:?}", $msg, $err)),
            #[cfg(feature = "warning")]
            LogType::Warning => println!("[{} {}] {:?}", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(), $enum.red(), format!("{} {:?}", $msg, $err)),
            LogType::Error => {
                println!("[{} {}] {:?}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(), $enum.red(), format!("{} {:?}", $msg, $err));
                panic!("{:?}", $err);
            },
            _ => (),
        };
    };
    ($enum:expr, $msg:expr) => {
        match $enum {
            #[cfg(feature = "debug")]
            LogType::Debug => println!("[{} {}] {:?}", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(), $enum.bright_yellow(), $msg),
            #[cfg(feature = "warning")]
            LogType::Warning => println!("[{} {}] {:?}", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(), $enum.red(), $msg),
            LogType::Error => {
                use chrono::Utc;
                println!("[{} {}] {:?}", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(), $enum.red(), $msg);
                panic!("{}", $msg);
            }
            _ => (),
        }
    };
}

#[derive(Debug)]
pub enum LogType {
    Warning,
    Debug,
    Error,
}