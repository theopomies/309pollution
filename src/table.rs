use std::{fs::read_to_string, str::FromStr};
use crate::binomial::Binomial;

pub struct Table {
    rows: Vec<Row>,
}

struct Row {
    x: u32,
    y: u32,
    pollution: u32,
}

impl Table {
    pub fn try_from_csv_file(file: String) -> Result<Self, String> {
        let rows = read_to_string(file)
            .map_err(|e| e.to_string())?
            .lines()
            .map(|line| Row::from_str(line))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(Self { rows })
    }

    pub fn apply_algo(&self, n: u32, x: f64, y: f64) -> f64 {
        let x = x / (n as f64 - 1.0);
        let y = y / (n as f64 - 1.0);
        let mut res = 0.0;

        for i in 0..n {
            for j in 0..n {
                if let Some(Row { pollution, .. }) = self.get_row(j, i) {
                    res += bp(n - 1, i, y) * bp(n - 1, j, x) * *pollution as f64
                }
            }
        }

        res
    }

    fn get_row(&self, x: u32, y: u32) -> Option<&Row> {
        self.rows.iter().find(|&r| r.x == x && r.y == y)
    }
}

impl FromStr for Row {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s
            .split(';')
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Invalid csv file.".to_owned())?[..]
        {
            &[x, y, pollution] => Ok(Row { x, y, pollution }),
            _ => Err("Invalid csv file.".to_owned()),
        }
    }
}

fn bp(n: u32, i: u32, u: f64) -> f64 {
    n.binomial(i) as f64 * u.powf(i.into()) * (1.0 - u).powf(n as f64 - i as f64)
}
