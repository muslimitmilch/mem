use std::io::stdout;
use std::io;
use std::io::Write;
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;


#[derive(Default)]
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
        Self::clear_screen();
        let raw_mode = stdout().into_raw_mode()?;
        Ok(Self {
            size: Size::default(),
            _raw_mode: raw_mode,
        })
    }

    pub fn set_size(&mut self) {
        let terminal_size = termion::terminal_size()
            .expect("wo terminal size");
        self.size = Size {
            width: terminal_size.0 as usize,
            height: terminal_size.1 as usize,
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn cursor_pos(x: usize, y: usize) {
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x + 1, y + 1));
    }

    pub fn flush() {
        io::stdout().flush().expect("error flushing stdout");
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn show_cursor() {
        print!("{}", termion::cursor::Show);
    }

    pub fn hide_cursor() {
        print!("{}", termion::cursor::Hide);
    }
}
