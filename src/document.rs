use std::fs;
use std::cmp;
use std::io;
use std::io::Write;



pub struct Document {
    rows: Vec<Row>,
    file_name: Option<String>,
}

impl Document {
    pub fn from(string: &str) -> Self {
        Self {
            rows: vec![Row::from(string)],
            file_name: None,
        }
    }

    pub fn open_file(filename: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in content.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
        })
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let mut file = fs::File::create(&self.file_name.clone().unwrap())?;
        for row in &self.rows {
            file.write_all(row.to_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(())
    }

    pub fn file_name(&self) -> &Option<String> {
        &self.file_name
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn delete_row(&mut self, index: usize) {
        self.rows.remove(index);
        if self.rows.len() == 0 {
            self.rows = vec![Row::from("")]
        }
    }

    pub fn insert_row(&mut self, index: usize) {
        let mut new_rows: Vec<Row> = Vec::new();
        for (i, item) in self.rows.iter().enumerate() {
            new_rows.push(item.clone());
            if i == index {
                new_rows.push(Row::from(""));
            }
        };
        self.rows = new_rows;
    }

    pub fn insert_char(&mut self, new_char: char, index_x: usize, index_y: usize) {
        self.rows[index_y].insert_char(new_char, index_x);
    }

    pub fn delete_char(&mut self, index_x: usize, index_y: usize) {
        self.rows
            .get_mut(index_y)
            .unwrap()
            .delete_char(index_x);
    }
}



#[derive(Clone)]
pub struct Row {
    string: String,
}

impl Row {
    fn from(slice: &str) -> Self {
        Self {string: String::from(slice)}
    }

    fn to_bytes(&self) -> &[u8] {
        self.string.as_bytes()
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

    pub fn delete_char(&mut self, index: usize) {
        let before: String = self.string
            .chars()
            .take(index)
            .collect();
        let after: String = self.string
            .chars()
            .skip(index + 1)
            .collect();
        self.string = before + &after;
    }

    pub fn len(&self) -> usize {
        self.string.chars().count()
    }
}
