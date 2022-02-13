use std::{fs::read_to_string, str::FromStr};

struct Row {
    x: u32,
    y: u32,
    pollution: u32,
}

pub struct Table {
    rows: Vec<Row>,
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
}
