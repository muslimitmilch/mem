use std::io::stdout;
use std::io;
use std::io::Write;
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;


pub struct Size {
    pub height: usize,
    pub width: usize,
}

pub struct Terminal {
    size: Size,
    _raw_mode: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Self::clear_screen();
        Ok(Self {
            size: Size {
                width: size.0 as usize,
                height: size.1 as usize,
            },
            _raw_mode: stdout().into_raw_mode().unwrap(),
        })
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn cursor_pos(&self, x: usize, y: usize) {
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x + 1, y + 1));
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

    pub fn show_cursor(&self) {
        print!("{}", termion::cursor::Show);
    }

    pub fn hide_cursor(&self) {
        print!("{}", termion::cursor::Hide);
    }
}
