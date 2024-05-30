use std::{
    env,
    error::Error,
    io::{self, BufRead, BufReader},
    fs::File,
};
use ccut::{Arguments, process_lines};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let Arguments {
        file_name,
        delimiter,
        fields_needed,
    } = Arguments::new(&args)?;

    let reader: Box<dyn BufRead> = match file_name.as_deref() {
        //* read from stdin
        Some("-") | None => Box::new(BufReader::new(io::stdin())),
        Some(file_name) => {
            let file = File::open(file_name)?;
            Box::new(BufReader::new(file))
        }
    };

    let mut stdout = io::stdout().lock();
    process_lines(reader, &mut stdout, &delimiter, &fields_needed)?;
    Ok(())
}
