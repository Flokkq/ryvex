#![cfg_attr(not(feature = "std"), no_std)]
pub extern crate alloc;

pub mod args;
pub mod commands;
pub mod compositor;
pub mod editor;
pub mod error;
pub mod keymap;
pub mod macros;
pub mod startup;
pub mod terminal_guard;
pub mod ui;
