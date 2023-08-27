use std::io;
use std::env;
use std::io::Write;
use termion::event::Key;

use crate::Document;
use crate::Terminal;

//const VERSION: &str = env!("CARGO_PKG_VERSION");



fn die(error: std::io::Error) {
    println!("{}", termion::clear::All);
    panic!("{}", error);
}


enum Mode {
    Insert,
    Normal,
    Prompt,
}


#[derive(Default)]
struct Position {
    x: usize,
    y: usize,
}

enum _ExitCode { // experiment; not implemented
    SaveExit,
    DelExit,
    Save,
    Nothing,
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
            match Document::open(&file_name) {
                Ok(document) => document,
                Err(_e) => Document::default(),
            }
        } else {
            Document::default()
        };
        Self {
            mode: Mode::Normal,
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
            if self.should_quit  {
                println!();
                break;
            }
            if let Err(error) = self.draw_screen() {
                die(error);
            }
            self.process_keypress();
        }
    }

    fn process_keypress(&mut self) {
        let key = Terminal::read_key().expect("unable to successfully read key");
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Esc => {
                self.mode = Mode::Normal;
                self.prompt_string = String::new();
            },
            Key::Char(c) => match self.mode {
                Mode::Normal => self.handle_key_command(c),
                Mode::Prompt => self.prompt(Key::Char(c)),
                Mode::Insert => self.insert_char(c),
            },
            Key::Backspace => match self.mode {
                Mode::Prompt => self.prompt(Key::Backspace),
                Mode::Insert => (), //to be handled
                _ => (),
            },
            _ => (),
        };
    }

    fn prompt(&mut self, key: Key) {
        match key {
            Key::Backspace => {
                self.prompt_string.pop();
            },
            Key::Char(c) => match c {
                '\n' => {
                    self.mode = Mode::Normal;
                    self.evaluate_prompt();
                },
                _ => self.prompt_string.push(c),
            },
            _ => (),
        };
    }

    fn insert_char(&mut self, character: char) {
        self.document.insert_char(character, self.cursor_pos.x, self.cursor_pos.y);
        self.cursor_pos.x += 1;
    }

    fn evaluate_prompt(&mut self) {
        let prompt = self.prompt_string.as_str();
        //let command: Vec<&str> = prompt.split_whitespace().collect();
        match prompt {
            "q" => self.should_quit = true,
            "wq" => { self.document.save().unwrap(); self.should_quit = true; },
            _ => (),
        }
        // tbc
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

    fn draw_bottom_line(&self) { // ERROR: STRING LENGTH / UNICODE
        let max_width = self.terminal.size().width as usize - 1;
        let left_text = String::from(" ") + &self.prompt_string;
        let middle_text = format!("{}", self.document.file_name());
        let right_text = match &self.mode {
            Mode::Insert => format!("INSERT"),
            Mode::Normal => format!("NORMAL"),
            Mode::Prompt => format!("PROMPT"),
        };
        //let right_text = format!("mem {}", VERSION);
        let left_padding_len = // UNDERFLOW
            max_width / 2 -
            left_text.chars().count() -
            (middle_text.len() / 2);
        let left_padding = " ".repeat(left_padding_len);
        let right_padding_len = // UNDERFLOW
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
        Terminal::clear_line();
        print!("{}", whole_line);
    }

    fn position_cursor(&mut self) -> Position {
        match self.mode {
            Mode::Prompt => return Position {
                x: 1 + self.prompt_string.chars().count(),
                y: self.terminal.size().height,
            },
            Mode::Insert => {
                self.cursor_pos.x = std::cmp::min(self.cursor_pos.x, match self.document.row(self.cursor_pos.y) {
                    Some(row) => row.len(),
                    None => 0,
                },)
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

    fn handle_key_command(&mut self, character: char) {
        match character {
            'i' => self.mode = Mode::Insert,
            'ö' => {
                self.mode = Mode::Prompt;
                self.prompt_string = String::new();
            },
            'h' => self.command_left(),
            'j' => self.command_down(),
            'k' => self.command_up(),
            'l' => self.command_right(),
            'x' => self.document.delete_char(self.cursor_pos.x, self.cursor_pos.y),
            'd' => self.document.delete_row(self.cursor_pos.y),
            'o' => self.document.insert_row(self.cursor_pos.y),
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
            if self.cursor_pos.x < row.len() { 
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
