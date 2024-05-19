use std::error::Error;

pub mod app;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
