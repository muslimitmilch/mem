use std::io::stdout;
use std::io;
use std::io::Write;
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;


pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    size: Size,
    _raw_mode: RawTerminal<std::io::Stdout>
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _raw_mode: stdout().into_raw_mode().unwrap(),
        })
    }

    pub fn clear_screen(&self) {
        println!("{}", termion::clear::All);
    }

    pub fn size(&self) -> (&Size) {
        &self.size
    }

    pub fn cursor_pos(&self, x: u16, y: u16) {
        println!("{}", termion::cursor::Goto(x + 1, y + 1));
    }

    pub fn flush(&self) {
        io::stdout().flush().expect("error flushing stdout");
    }

    pub fn read_key(&self) -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
