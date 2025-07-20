#[macro_export]
macro_rules! log {
    ($lvl:expr, $($arg:tt)*) => {{
        if $crate::logging::LOGGER.enabled($lvl) {
            $crate::logging::log_msg_internal(
                $lvl,
                core::format_args!($($arg)*),
                core::module_path!(),
                file!(),
                line!()
            );
        }
    }};
}

#[macro_export]
macro_rules! info  { ($($t:tt)*) => { $crate::log!($crate::logging::record::LogLevel::Info,  $($t)*) }; }
#[macro_export]
macro_rules! warn  { ($($t:tt)*) => { $crate::log!($crate::logging::record::LogLevel::Warn,  $($t)*) }; }
#[macro_export]
macro_rules! error { ($($t:tt)*) => { $crate::log!($crate::logging::record::LogLevel::Error, $($t)*) }; }
#[macro_export]
macro_rules! debug { ($($t:tt)*) => { $crate::log!($crate::logging::record::LogLevel::Debug, $($t)*) }; }
#[macro_export]
macro_rules! trace { ($($t:tt)*) => { $crate::log!($crate::logging::record::LogLevel::Trace, $($t)*) }; }

#[macro_export]
macro_rules! log_error_chain {
    ($err:expr) => {
        $crate::logging::log_error_chain(
            $err,
            None,
            core::module_path!(),
            file!(),
            line!()
        )
    };
    ($err:expr, $($ctx:tt)+) => {
        $crate::logging::log_error_chain(
            $err,
            Some(core::format_args!($($ctx)+)),
            core::module_path!(),
            file!(),
            line!()
        )
    };
}
