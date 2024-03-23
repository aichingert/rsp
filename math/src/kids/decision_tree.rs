use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

// This implementation is making a lot of assumptions 
pub struct DTree {
    rows: Vec<Column>,
}

impl DTree {
    pub fn new(path: impl AsRef<Path>) -> Option<Self> {
        let mut iter = BufReader::new(File::open(path).ok()?).lines();
        let mut rows: Vec<Column> = iter.next()?.ok()?
            .split(';')
            .map(|label| Column::new(label.to_string()))
            .collect();

        for line in iter {
            for (i, col) in line.ok()?.split(';').enumerate() {
                rows[i].push(col == "True" || col == "Negative");
            }
        }

        Some(Self { rows }) 
    }

    pub fn generate(&self) {
        let target = &self.rows[self.rows.len() - 1];
        let parent_entropy = target.entropy();

        for i in 0..self.rows.len() - 1 {
            let splits = Column::split(&self.rows[i], vec![target]);
            let (y, n) = splits.first().unwrap();

            let len = self.rows[i].entries.len() as f32;
            let malig = y.entries.len() as f32;
            let benig = n.entries.len() as f32;

            let split = (malig / len) * y.entropy() + (benig / len) * n.entropy();
            let infog = parent_entropy - split;
            println!("{}: {infog}", y.label);
        }
    }
}

#[derive(Debug)]
struct Column {
    label: String,
    entries: Vec<bool>,
}

impl Column {
    fn new(label: String) -> Self {
        Self { label, entries: Vec::new() }
    }

    fn push(&mut self, entry: bool) {
        self.entries.push(entry);
    }

    fn entropy(&self) -> f32 {
        let len = self.entries.len() as f32;
        let malig = self.entries.iter().filter(|&e| *e).count() as f32;
        let benig = len - malig;

        if malig == 0. || benig == 0. { return 0. }

        -((malig / len) * (malig / len).log2() + (benig / len) * (benig / len).log2())
    }

    fn split(on: &Column, columns: Vec<&Column>) -> Vec<(Column, Column)> {
        let mut split = Vec::new();

        for column in columns {
            let mut malig = Column::new(format!("split_on_{}_m", on.label));
            let mut benig = Column::new(format!("split_on_{}_b", on.label));

            for (o_r, c_r) in on.entries.iter().zip(column.entries.iter()) {
                match o_r {
                    true  => malig.push(*c_r),
                    false => benig.push(*c_r),
                }
            }

            split.push((malig, benig));
        }

        split
    }
}
