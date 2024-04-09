use std::error::Error;

pub mod app;
pub mod key;
pub mod text;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
