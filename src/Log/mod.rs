/// This is a custom logging macro for the library.
/// # Examples
/// ```rust
/// use fish::{log, Log::{LogType}};
/// log!(LogType::Debug, "The player moved");
/// ```
#[macro_export]
macro_rules! log {
    ($enum:expr, $msg:expr, $err:expr) => {
<<<<<<< HEAD

=======
>>>>>>> 307746b805f2c3b2d256a2c585f623996ff6afb5
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