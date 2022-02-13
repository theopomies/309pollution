use std::process::exit;

mod args;
mod binomial;
mod table;

use args::{Arguments, Commands};
use table::Table;

const HELP_MESSAGE: &str = "USAGE
\t./308reedpipes r0 r5 r10 r15 r20 n
DESCRIPTION
\tr0\tradius (in cm) of pipe at the 0cm abscissa
\tr5\tradius (in cm) of pipe at the 5cm abscissa
\tr10\tradius (in cm) of pipe at the 10cm abscissa
\tr15\tradius (in cm) of pipe at the 15cm abscissa
\tr20\tradius (in cm) of pipe at the 20cm abscissa
\tn\tnumber of points needed to display the radius";

fn main() {
    match Commands::try_from_args() {
        Err(e) => {
            eprintln!("{}", HELP_MESSAGE);
            eprintln!("{}", e);
            exit(84)
        }
        Ok(Commands::Help) => {
            println!("{}", HELP_MESSAGE);
            exit(0)
        }
        Ok(Commands::Program(arguments)) => {
            if let Err(e) = program(arguments) {
                eprintln!("{}", e);
                exit(84)
            }
        }
    }
}

fn program(arguments: Arguments) -> Result<(), String> {
    let table = Table::try_from_csv_file(arguments.file)?;
    let result = table.apply_algo(arguments.n, arguments.x, arguments.y);
    println!("{:.2}", result);
    Ok(())
}
