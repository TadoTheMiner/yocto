use std::{error::Error, io::Stderr};

use ratatui::backend::CrosstermBackend;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stderr>>;
