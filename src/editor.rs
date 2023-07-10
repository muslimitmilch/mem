use std::io;
use std::env;
use std::io::Write;
use termion::event::Key;

use crate::Document;
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


#[derive(Default)]
struct Position {
    x: usize,
    y: usize,
}


pub struct Editor {
    mode: Mode,
    should_quit: bool,
    terminal: Terminal,
    document: Document,
    cursor_pos_in_doc: Position,
    cursor_y_on_screen: usize,
}

impl Editor {
    pub fn default() -> Self{
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };
        Self {
            mode: Mode::Bereit,
            should_quit: false,
            terminal: Terminal::default().expect("wo terminal"),
            document,
            cursor_pos_in_doc: Position::default(),
            cursor_y_on_screen: 0,
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
                Key::Char(c) => (), //tbc
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
            self.draw_rows(self.terminal.size().height - 1);
            self.draw_bottom_line();
            self.terminal.show_cursor();
            self.terminal.cursor_pos(0, 0);
        }
        io::stdout().flush()
    }
    
    fn draw_rows(&self, height: usize) {
        for y in 0..height {
            let option_row = self.document.row(y);
            let string = match option_row {
                Some(row) => row.render(),
                None => "~".to_string(),
            };
            println!("{}\r", string);
        }
    }

    fn draw_bottom_line(&self) {
        let max_width = self.terminal.size().width as usize;
        let left_text = match &self.mode {
            breit => format!(" breit"),
            bereit => format!(" bereit"),
        };
        let middle_text = self.document.file_name();
        let right_text = format!("{}", VERSION);
        let padding_len =
            max_width -
            left_text.len() -
            middle_text.len() -
            right_text.len();
        let padding = " ".repeat(padding_len / 2);
        let whole_line =
            format!("{}{}{}{}{}",
            left_text,
            padding,
            middle_text,
            padding,
            right_text
            );
        print!("{}", whole_line);
    }
}
