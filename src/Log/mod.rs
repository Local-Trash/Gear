/// This is a custom logging macro for the library.
/// # Examples
/// ```rust
/// 
/// ```
#[macro_export]
macro_rules! log {
    ($enum:expr, $msg:expr, $err:expr) => {

        match $enum {
            #[cfg(feature = "debug")]
            LogType::Debug => println!("[{} {}] {:?}", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(), $enum.bright_yellow(), format!("{} {}", $msg, $err)),
            #[cfg(feature = "warning")]
            LogType::Warning => println!("[{} {}] {:?}", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(), $enum.red(), format!("{} {}", $msg, $err)),
            LogType::Error => {
                #[cfg(feature = "error")]
                println!("[{} {}] {:?}", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(), $enum.red(), format!("{} {}", $msg, $err));
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
                #[cfg(feature = "error")]
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