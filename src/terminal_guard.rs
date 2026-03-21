use std::io::stdout;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::DefaultTerminal;

pub struct TerminalGuard {
    pub terminal: DefaultTerminal,
}

impl TerminalGuard {
    pub fn new() -> Result<Self, std::io::Error> {
        let terminal = ratatui::init();
        crossterm::execute!(stdout(), EnableMouseCapture)?;
        Ok(Self { terminal })
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = crossterm::execute!(stdout(), DisableMouseCapture);
        ratatui::restore();
    }
}
