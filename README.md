# ryvex

ryvex is a lightweight, Vim-like text editor built in Rust. This project aims to provide a simple yet powerful text editing experience in the terminal, drawing inspiration from the classic Vim editor.

## Dependencies

ryvex relies on minimal external dependencies to maintain its lightweight nature. The primary dependency is `termios`, a Rust crate providing an interface for the POSIX termios API. This dependency is crucial for manipulating terminal settings directly, enabling features like raw mode, which is essential for a terminal-based text editor. Raw mode allows the editor to handle key presses and terminal behavior at a low level, similar to Vim.
