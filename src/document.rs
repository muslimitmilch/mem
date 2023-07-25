use std::fs;
use std::cmp;



#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    file_name: String,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in content.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: filename.to_string(),
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
       self.rows.get(index)
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn rows(&self) -> &Vec<Row> {
       &self.rows
    }

    pub fn insert_char(&mut self, new_char: char, index_x: usize, index_y: usize) {
        if index_y <= self.rows.len() {
            self.rows[index_y].insert_char(new_char, index_x)
        } else {
            self.rows.push(Row::from(&new_char.to_string()))
        }
    }
}



pub struct Row {
    string: String,
}

impl Row {
    fn from(slice: &str) -> Self {
        Self {string: String::from(slice)}
    }

    //legacy
    pub fn string(&self) -> &String {
        &self.string
    }

    pub fn render(&self, mut begin: usize, mut end: usize) -> String {
        end = cmp::min(end, self.len());
        begin = cmp::min(begin, end);
        let mut output_string = String::new();
        for character in self.string
            .chars()
            .skip(begin)
            .take(end - begin)
        {
            if character == '\t' {
                output_string.push_str("    ");
            } else {
                output_string.push(character);
            }
        };
        output_string
    }

    pub fn insert_char(&mut self, new_char: char, index: usize) {
        let before: String = self.string
            .chars()
            .take(index)
            .collect();
        let after: String = self.string
            .chars()
            .skip(index)
            .collect();
        self.string = before + &new_char.to_string() + &after;
    }

    pub fn len(&self) -> usize {
        self.string.chars().count()
    }
}
