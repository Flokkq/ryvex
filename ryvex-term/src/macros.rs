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
macro_rules! queue {
    ($writer:expr $(, $command:expr)* $(,)?) => {{
        use ::std::io::Write;

        Ok($writer.by_ref())
            $(.and_then(|writer| $crate::command::QueueableCommand::queue(writer, $command)))*
            .map(|_| ())
    }}
}

#[macro_export]
macro_rules! execute {
    ($writer:expr $(, $command:expr)* $(,)? ) => {{
        use ::std::io::Write;

        // Queue each command, then flush
        $crate::queue!($writer $(, $command)*)
            .and_then(|()| {
                ::std::io::Write::flush($writer.by_ref())
            })
    }}
}
