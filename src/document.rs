pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::from("hello world"));
        rows.push(Row::from("zweite zeile"));
        Self {rows}
    }

    pub fn row(&self, index: usize) -> &Row {
       &self.rows[index]
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
    pub fn string(&self) -> &String {
        &self.string
    }
}
