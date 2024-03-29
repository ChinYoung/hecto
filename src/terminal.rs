use std::io;

use termion::{event::Key, input::TermRead};

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    pub size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
