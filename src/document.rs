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
        end = cmp::min(end, self.string.len());
        begin = cmp::min(begin, end);
        self.string
            .get(begin..end)
            .unwrap_or_default()
            .to_string()
    }
}
