use std::io;
use std::io::stdout;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;


//legacy
fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(error: std::io::Error) {
    panic!("{}", error);
}

enum Mode {
    Breit,
    Bereit,
}

pub struct Editor {
    mode: Mode,
}

impl Editor {
    pub fn default() -> Self{
        Self{mode: Mode::Bereit}
    }

    pub fn run(&self) {
        let _stdout_raw_mode = stdout().into_raw_mode().unwrap();

        for key_result in io::stdin().keys() {
            match key_result {
                Ok(key) => match key {
                    Key::Char(c) => println!("{}\r", c),
                    Key::Ctrl('q') => break,
                    _ => println!("{:?}\r", key),
                    },
                Err(err) => die(err),
            }
        }
    }
}
