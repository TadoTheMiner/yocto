use std::io::Stderr;

use ratatui::backend::CrosstermBackend;
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stderr>>;
