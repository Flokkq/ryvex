#[macro_export]
#[doc(hidden)]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

#[macro_export]
#[doc(hidden)]
macro_rules! osc {
    ($( $l:expr ),*) => { concat!("\x1B]", $( $l ),*, "\x1B\\") };
}

#[macro_export]
macro_rules! c {
	($l:expr) => {
		concat!($l, "\0").as_ptr() as *const ::core::ffi::c_char
	};
}

#[macro_export]
macro_rules! queue {
    ($writer:expr $(, $command:expr)* $(,)?) => {{
        use $crate::std::write::Write;

        Ok($writer.by_ref())
            $(.and_then(|writer| $crate::term::command::QueueableCommand::queue(writer, $command).map_err($crate::std::error::IoError::from)))*
            .map(|_| ())
    }}
}

#[macro_export]
macro_rules! execute {
    ($writer:expr $(, $command:expr)* $(,)? ) => {{
        use $crate::std::write::Write;

        // Queue each command, then flush
        $crate::queue!($writer $(, $command)*)
            .and_then(|()| {
                $crate::std::write::Write::flush($writer.by_ref()).map_err($crate::std::error::IoError::from)
            })
    }}
}
