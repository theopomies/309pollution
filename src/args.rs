use std::str::FromStr;

pub enum Commands {
    Program(Arguments),
    Help,
}

#[derive(Debug)]
pub struct Arguments {
    pub n: u32,
    pub file: String,
    pub x: f64,
    pub y: f64,
}

impl Commands {
    pub fn try_from_args() -> Result<Commands, String> {
        let mut args = std::env::args().skip(1);
        if args.any(|arg| arg == "-h" || arg == "--help") {
            return Ok(Commands::Help);
        }
        let mut args = std::env::args().skip(1);
        let n = parse_from_iterator(&mut args)?;
        if n == 0 {
            return Err("N can't be zero.".into());
        }
        let file = args.next().ok_or("Invalid argument count.".to_owned())?;
        let x = parse_from_iterator(&mut args)?;
        let y = parse_from_iterator(&mut args)?;

        if let Some(_) = args.next() {
            return Err("Invalid argument count.".into());
        }

        Ok(Commands::Program(Arguments { n, file, x, y }))
    }
}

fn parse_from_iterator<T: FromStr>(iter: &mut impl Iterator<Item = String>) -> Result<T, String>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    iter.next()
        .map(|arg| arg.parse::<T>())
        .ok_or("Invalid argument count.".to_owned())?
        .map_err(|e| e.to_string())
}
