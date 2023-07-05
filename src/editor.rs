use std::io;
use std::io::Write;
use termion::event::Key;

use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");



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
        if self.should_quit {
            self.terminal.clear_screen();
            println!(" tschö mit ö\r")
        } else {
            self.terminal.hide_cursor();
            self.draw_rows();
            self.draw_bottom_line();
            self.terminal.cursor_pos(0, 0);
            self.terminal.show_cursor();
        }
        io::stdout().flush()
    }
    
    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
           self. terminal.clear_line();
            println!("~\r");
        }
    }

    fn draw_bottom_line(&self) {
        let max_width = self.terminal.size().width as usize;
        let left_text = match &self.mode {
            breit => format!(" breit"),
            bereit => format!(" bereit"),
        };
        let middle_text = format!("{}", VERSION);
        let right_text = format!("mem_editor");
        let padding_len = max_width - left_text.len() - middle_text.len() - right_text.len();
        let padding = " ".repeat(padding_len / 2);
        let whole_line = format!("{}{}{}{}{}", left_text, padding, middle_text, padding, right_text);
        print!("{}", whole_line);
    }
}
