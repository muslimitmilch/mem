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
    cursor_pos: Position,
    scroll: Position,
}

impl Editor {
    pub fn default() -> Self{
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).expect("err opening file")
        } else {
            Document::default()
        };
        Self {
            mode: Mode::Bereit,
            should_quit: false,
            terminal: Terminal::default().expect("wo terminal"),
            document,
            cursor_pos: Position::default(), //cursor pos in doc
            scroll: Position::default(),
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
            self.process_keypress();
        }
    }

    fn process_keypress(&mut self) {
        let key = Terminal::read_key().expect("unable to successfully read key");
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => self.handle_key_command(key),
        };
    }

    fn draw_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen();
            println!(" tschö mit ö\r")
        } else {
            self.terminal.set_size();
            Terminal::cursor_pos(0, 0);
            let position = self.position_cursor();
            Terminal::hide_cursor();
            self.draw_rows();
            self.draw_bottom_line();
            Terminal::cursor_pos(position.x, position.y);
            //print!("x{} {}", position.x, self.cursor_pos.x);
        }
        Terminal::show_cursor();
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        for y in self.scroll.y .. self.scroll.y + self.terminal.size().height - 1 {
            let option_row = self.document.row(y);
            let string = match option_row {
                Some(row) => row
                .render(self.scroll.x,
                    self.scroll.x + self.terminal.size().width),
                None => "~".to_string(),
            };
            Terminal::clear_line();
            println!("{}\r", string);
        }
    }

    fn draw_bottom_line(&self) {
        let max_width = self.terminal.size().width as usize;
        let left_text = match &self.mode {
            breit => format!(" breit"),
            bereit => format!(" bereit"),
        };
        let middle_text = format!("{}", self.document.file_name());
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

    fn position_cursor(&mut self) -> Position {
        if self.cursor_pos.x >
            self.scroll.x + self.terminal.size().width - 1 {
            self.scroll.x += 1;
        } else if self.cursor_pos.x < self.scroll.x {
            self.scroll.x -= 1;
        }
        if self.cursor_pos.y >
            self.scroll.y + self.terminal.size().height - 2 {
            self.scroll.y += 1;
        } else if self.cursor_pos.y < self.scroll.y {
            self.scroll.y -= 1;
        }
        Position {
            x: self.cursor_pos.x - self.scroll.x,
            y: self.cursor_pos.y - self.scroll.y,
        }
    }

    fn handle_key_command(&mut self, key: Key) {
        match key {
            Key::Char('h') => self.command_left(),
            Key::Char('j') => self.command_down(),
            Key::Char('k') => self.command_up(),
            Key::Char('l') => self.command_right(),
            _ => (),
        };
    }

    fn command_down(&mut self) {
        let cpid = self.cursor_pos.y;
        match self.document.row(cpid + 1) {
            Some(row) => self.cursor_pos.y += 1,
            _ => (),
        }
    }

    fn command_right(&mut self) {
        match self.document.row(self.cursor_pos.y) {
            Some(row) =>
            if self.cursor_pos.x < row.string().len() { 
                self.cursor_pos.x += 1;
            },
            _ => (),
        }
    }

    fn command_up(&mut self) {
        if self.cursor_pos.y != 0 {
            self.cursor_pos.y -= 1;
        };
    }

    fn command_left(&mut self) {
        if self.cursor_pos.x != 0 {
            self.cursor_pos.x -= 1;
        };
    }
}
