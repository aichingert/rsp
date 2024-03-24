use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

// This implementation is making a lot of assumptions 
// 
// str -> true {
//  return x;
// } else {
//  return y;
// }
//

pub struct DecisionTree {
    condition: String,
    true_branch: Option<Box<DecisionTree>>,
    false_branch: Option<Box<DecisionTree>>,
}

impl DecisionTree {
    pub fn from(path: impl AsRef<Path>) -> Option<Self> {
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

        Self::generate(rows)
    }

    fn generate(rows: Vec<Column>) -> Option<Self> {
        if rows.is_empty() {
            return None;
        }

        let len = rows.len();
        let target = &rows[len - 1];
        let parent_entropy = target.entropy();

        let mut idx = 0;
        let mut igb = 0.;

        for (i, row) in rows.iter().take(len - 1).enumerate() {
            let (y, n) = Column::split(row, target);

            let len = rows[i].entries.len() as f32;
            let malig = y.entries.len() as f32;
            let benig = n.entries.len() as f32;

            let split = (malig / len) * y.entropy() + (benig / len) * n.entropy();
            let infog = parent_entropy - split;

            println!("{} -> {infog} | {split}", row.label);
            if infog > igb {
                idx = i;
                igb = infog;
            }
        }

        /*
        let (mut td, mut fd) = (true, true);

        for (&s, &t) in rows[idx].entries.iter().zip(target.entries.iter()) {
            if s != t {
                if s {
                    td = false;
                } else {
                    fd = false;
                }
            }
        }

        let (tb, fb) = Column::seperate(&rows[idx], &rows);

        let true_branch = if rows.len() == 2 || td { None }
        else { Self::generate(tb).map_or(None, |dt| Some(Box::new(dt))) };

        let false_branch = if rows.len() == 2 || fd { None }
        else { Self::generate(fb).map_or(None, |dt| Some(Box::new(dt))) };
        */

        Some(Self {
            condition: rows[idx].label.clone(),
            true_branch: None,
            false_branch: None
        })
    }

    pub fn print_tree(&self, ident: &str) {
        println!("{ident}if {} {{", self.condition);

        let mut ident = if ident == "" {
            String::from("----")
        } else {
            format!("{ident}----")
        };

        if let Some(t) = self.true_branch.as_ref() {
            t.print_tree(&ident);
        } else {
            println!("{ident}true");
        }

        if let Some(f) = self.false_branch.as_ref() {
            f.print_tree(&ident);
        } else {
            println!("{ident}false");
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

    fn split(on: &Column, target: &Column) -> (Column, Column) {
        let mut malig = Column::new(format!("split_on_{}_y", on.label));
        let mut benig = Column::new(format!("split_on_{}_n", on.label));

        for (o_r, c_r) in on.entries.iter().zip(target.entries.iter()) {
            match o_r {
                true  => malig.push(*c_r),
                false => benig.push(*c_r),
            }
        }

        (malig, benig)
    }

    fn seperate(on: &Column, rows: &[Column]) -> (Vec<Column>, Vec<Column>) {
        let (mut malig, mut benig) = (Vec::new(), Vec::new());

        for row in rows.iter() {
            if row.label == on.label {
                continue;
            }

            let mut column_m = Column::new(row.label.clone());
            let mut column_b = Column::new(row.label.clone());

            for (o_r, c_r) in on.entries.iter().zip(row.entries.iter()) {
                match o_r {
                    true => column_m.push(*c_r),
                    false => column_b.push(*c_r),
                }
            }

            malig.push(column_m);
            benig.push(column_b);
        }

        (malig, benig)
    }
}
