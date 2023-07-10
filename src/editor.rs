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



#[derive(Default)]
struct ScreenExerpt {
    begin: Position,
    end: Position,
}



pub struct Editor {
    mode: Mode,
    should_quit: bool,
    terminal: Terminal,
    document: Document,
    cursor_pos_in_doc: Position,
    screen_excerpt: ScreenExerpt,
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
            screen_excerpt: ScreenExerpt::default(),
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
                Key::Ctrl('q') => self.should_quit = true,
                _ => self.handle_key_command(key),
                },
            Err(err) => die(err),
        }
        Ok(())
    }

    fn draw_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen();
            println!(" tschö mit ö\r")
        } else {
            self.terminal.cursor_pos(0, 0);
            self.terminal.hide_cursor();
            self.get_screen_excerpt();
            self.draw_rows();
            self.draw_bottom_line();
            let cpid = &self.cursor_pos_in_doc;
            self.terminal.cursor_pos(cpid.x, cpid.y);
        }
        self.terminal.show_cursor();
        io::stdout().flush()
    }

    fn get_screen_excerpt(&mut self) {
        self.screen_excerpt.end.y =
        self.screen_excerpt.begin.y +
        self.terminal.size().height - 1;
    }
    
    fn draw_rows(&self) {
        for y in self.screen_excerpt.begin.y ..
            self.screen_excerpt.end.y {
            let option_row = self.document.row(y);
            let string = match option_row {
                Some(row) => row.render(),
                None => "~".to_string(),
            };
            Terminal::clear_line;
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
        let right_text = format!("mem {}", VERSION);
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

    fn handle_key_command(&mut self, key: Key) {
        match key {
            //Key::Char('h') => self.command_left(),
            Key::Char('j') => self.command_down(),
            Key::Char('k') => self.command_up(),
            //Key::Char('l') => self.command_right(),
            _ => (),
        }
    }

    fn command_down(&mut self) {
        let cpid = self.cursor_pos_in_doc.y;
        match self.document.row(cpid + 1) {
            Some(row) => self.cursor_pos_in_doc.y += 1,
            _ => (),
        }
    }

    fn command_up(&mut self) {
        if self.cursor_pos_in_doc.y != 0 {
            self.cursor_pos_in_doc.y -= 1;
        };
    }
}
