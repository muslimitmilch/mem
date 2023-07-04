use std::io;
use std::io::Write;
use termion::event::Key;

use crate::Terminal;


fn die(error: std::io::Error) {
    println!("{}", termion::clear::All);
    panic!("{}", error);
}


enum Mode {
    Breit,
    Bereit,
}


struct Position {
    x: usize,
    y: usize,
}


pub struct Editor {
    mode: Mode,
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self{
        Self {
            mode: Mode::Bereit,
            should_quit: false,
            terminal: Terminal::default().expect("wo terminal"),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.draw_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = self.terminal.read_key();
        match pressed_key {
            Ok(key) => match key {
                Key::Char(c) => println!("{}\r", c),
                Key::Ctrl('q') => self.should_quit = true,
                _ => println!("{:?}\r", key),
                },
            Err(err) => die(err),
        }
        Ok(())
    }

    fn draw_screen(&self) -> Result<(), std::io::Error> {
        self.terminal.clear_screen();
        if self.should_quit {
            println!(" tschö mit ö\r")
        } else {
            self.draw_rows();
            self.terminal.cursor_pos(0, 0);
        }
        io::stdout().flush()
    }
    
    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
}
