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
    Prompt,
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
    prompt_string: String,
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
            prompt_string: String::new(),
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
            Key::Ctrl('a') => self.mode = Mode::Bereit,
            Key::Ctrl('f') => {
                self.mode = Mode::Prompt;
                self.prompt_string = String::from("");
            },
            _ => match self.mode {
                Mode::Bereit => self.handle_key_command(key),
                Mode::Prompt => self.prompt(key),
                Mode::Breit => self.insert_key(key),
            }
        };
    }

    fn prompt(&mut self, key: Key) {
        match key {
            Key::Esc => {
                self.mode = Mode::Bereit;
                self.prompt_string = String::new();
            },
            Key::Char(c) => match c {
                '\n' => {
                    self.mode = Mode::Bereit;
                    self.evaluate_prompt();
                },
                _ => self.prompt_string.push(c),
            }
            _ => (),
        };
    }

    fn insert_key(&self, key: Key) {
    }

    fn evaluate_prompt(&mut self) {
        if self.prompt_string.len() == 1 {
            self.handle_key_command(Key::Char(self.prompt_string.chars().next().unwrap()));
        }
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

    fn draw_bottom_line(&self) { // STILL ERROR PRONE
        let max_width = self.terminal.size().width as usize - 1;
        let left_text = String::from(" ") + &self.prompt_string;
        let middle_text = format!("{}", self.document.file_name());
        let right_text = match &self.mode {
            Mode::Breit => format!("BREIT"),
            Mode::Bereit => format!("BEREIT"),
            Mode::Prompt => format!("PROMPT"),
        };
        //let right_text = format!("mem {}", VERSION);
        let left_padding_len =
            max_width / 2 -
            left_text.len() -
            (middle_text.len() / 2);
        let left_padding = " ".repeat(left_padding_len);
        let right_padding_len =
            max_width / 2 -
            right_text.len() -
            (middle_text.len() / 2);
        let right_padding = " ".repeat(right_padding_len);
        let whole_line =
            format!("{}{}{}{}{}",
            left_text,
            left_padding,
            middle_text,
            right_padding,
            right_text
            );
        print!("{}", whole_line);
    }

    fn position_cursor(&mut self) -> Position {
        match self.mode {
            Mode::Prompt => return Position {
                x: 1 + self.prompt_string.len(),
                y: self.terminal.size().height,
            },
            _ => (),
        }
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
            Some(_row) => self.cursor_pos.y += 1,
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
