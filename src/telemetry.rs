use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, Once};

enum LogLevel {
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogLevel::Info => "INFO",
                LogLevel::Warn => "WARN",
                LogLevel::Error => "ERROR",
            }
        )
    }
}

struct Logger {
    file: File,
}

impl Logger {
    fn new() -> io::Result<Self> {
        let path = Path::new("logs");
        std::fs::create_dir_all(path)?;
        let file_path = path.join("log.txt");
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        Ok(Logger { file })
    }

    fn log(&mut self, level: LogLevel, msg: &str) {
        if let Err(e) = writeln!(self.file, "[{}] {}", level, msg) {
            eprintln!("Failed to write to log file: {}", e);
        }
    }
}

impl fmt::Debug for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Logger")
    }
}

#[derive(Clone)]
pub struct SingletonLogger {
    inner: Arc<Mutex<Logger>>,
}

impl SingletonLogger {
    pub fn get_instance() -> Self {
        static ONCE: Once = Once::new();
        static mut SINGLETON: Option<SingletonLogger> = None;

        unsafe {
            ONCE.call_once(|| {
                let logger =
                    Logger::new().expect("Failed to initialize logger");
                let singleton = SingletonLogger {
                    inner: Arc::new(Mutex::new(logger)),
                };
                SINGLETON = Some(singleton);
            });
            SINGLETON.clone().unwrap()
        }
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }

    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }

    fn log(&self, level: LogLevel, msg: &str) {
        if let Ok(mut logger) = self.inner.lock() {
            logger.log(level, msg);
        } else {
            eprintln!("Logger lock error");
        }
    }
}
