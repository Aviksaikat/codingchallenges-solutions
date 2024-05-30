use std::{
    error::Error,
    io::{BufRead, Write},
};

pub struct Arguments {
    pub file_name: Option<String>,
    pub delimiter: String,
    pub fields_needed: Vec<usize>,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Self, Box<dyn Error>> {
        let mut fields_needed = vec![];
        let mut delimiter = None;
        let mut file_name = None;
        const SKIP_CHALLENGE_PATH: usize = 1;
        // println!("{:?}", args.iter());
        //* Skip binary name that is target/debug/<name>
        let args = args.iter().skip(SKIP_CHALLENGE_PATH);
        const FIELD_COMMAND: &str = "-f";
        const DELIMITER_COMMAND: &str = "-d";
        let mut next_arg_is_value = false;
        let mut current_command = "";
        for arg in args {
            if arg.starts_with(FIELD_COMMAND) {
                current_command = FIELD_COMMAND;
                //* if passed -f 1 or -d 1 or -f"1,2" or -f "1,2" etc.
                if arg.len() > FIELD_COMMAND.len() {
                    let arg = arg.replace(FIELD_COMMAND, "");
                    fields_needed = Self::get_fields_needed(&arg)?;
                } else {
                    next_arg_is_value = true;
                }
            } else if arg.starts_with(DELIMITER_COMMAND) {
                current_command = DELIMITER_COMMAND;
                if arg.len() > DELIMITER_COMMAND.len() {
                    delimiter = Some(arg.replace(DELIMITER_COMMAND, ""));
                } else {
                    next_arg_is_value = true;
                }
            } else if next_arg_is_value {
                match current_command {
                    FIELD_COMMAND => fields_needed = Self::get_fields_needed(arg)?,
                    DELIMITER_COMMAND => delimiter = Some(arg.to_string()),
                    _ => unreachable!(),
                }
                next_arg_is_value = false;
            } else if !arg.trim().is_empty() {
                file_name = Some(arg.to_string());
            }
        }
        if fields_needed.is_empty() {
            return Err("No fields were provided".into());
        }
        let delimiter = delimiter.unwrap_or_else(|| "\t".to_string());
        Ok(Self {
            file_name,
            delimiter,
            fields_needed,
        })
    }

    fn get_fields_needed(fields_needed: &str) -> Result<Vec<usize>, Box<dyn Error>> {
        let results = if fields_needed.contains(',') {
            fields_needed
                .split(',')
                .map(|field| field.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?
        } else if fields_needed.contains(' ') {
            fields_needed
                .split(' ')
                .map(|field| field.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?
        } else {
            vec![fields_needed.parse::<usize>()?]
        };

        // we are subtracting 1 because the command is 1 based and the field is 0 based
        Ok(results.into_iter().map(|field| field - 1).collect())
    }
}

pub fn process_lines<R: BufRead, W: Write>(
    reader: R,
    writer: &mut W,
    delimiter: &str,
    fields_needed: &[usize],
) -> Result<(), Box<dyn Error>> {
    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(delimiter).collect();
        let mut first_field = true;
        for field_needed in fields_needed {
            if let Some(field) = fields.get(*field_needed) {
                if !first_field {
                    write!(writer, "{}", delimiter)?;
                }
                write!(writer, "{}", field)?;
                first_field = false;
            }
        }
        writeln!(writer)?;
    }
    Ok(())
}
