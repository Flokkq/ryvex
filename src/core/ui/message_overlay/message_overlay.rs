pub enum MessageLevel {
    Info,
    Warning,
    Error,
}

impl MessageLevel {
    pub fn to_color(&self) -> (&str, &str) {
        let (border_color, text_color) = match self {
            MessageLevel::Info => ("\x1b[34m", "\x1b[0m"), // blue border, default text color
            MessageLevel::Warning => ("\x1b[33m", "\x1b[33m"), // yellow border, default text color
            MessageLevel::Error => ("\x1b[31m", "\x1b[31m"), // red border, red text color
        };

        return (border_color, text_color);
    }
}
