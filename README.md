# ryvex

REWRITE SOON :tm:

ryvex is a lightweight, Vim-like text editor built in Rust. This project aims to provide a simple yet powerful text editing experience in the terminal, drawing inspiration from the classic Vim editor.

## Dependencies

ryvex relies on minimal external dependencies to maintain its lightweight nature. Below are the primary dependencies:

- `termios`: a Rust crate providing an interface for the POSIX termios API. This dependency is crucial for manipulating terminal settings directly, enabling features like raw mode. Raw mode allows the editor to handle key presses and terminal behavior at a low level, similar to Vim.

- `libc`: used in ryvex for accessing various low-level features provided by the operating system, including functions to determine the terminal width. This is essential for dynamic screen resizing and layout adjustments in the editor.

By keeping dependencies limited to essential system-level interfaces, ryvex ensures a robust and efficient text editing experience in the terminal.
