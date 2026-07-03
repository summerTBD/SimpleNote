#![warn(clippy::all, rust_2018_idioms)]

mod action;
mod app;
mod board;
mod note;
pub use action::Action;
pub use app::SimpleNoteApp;
pub use board::Board;
pub use note::Note;
