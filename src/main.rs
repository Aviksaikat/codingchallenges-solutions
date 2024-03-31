use std::env::args;
use std::fs::{metadata, read_to_string, File};
use std::io::{BufRead, BufReader};

pub fn count_multi_bytes(path: &str) -> String {
    let content = read_to_string(path);
    let char_count = content.expect("REASON").as_bytes().len();
    let char_count_str = char_count.to_string();

    let chars_str = format!("{:>pd$}", char_count_str, pd = char_count_str.len() + 1);
    chars_str
}
pub fn count_words(path: &str) -> u128 {
    let content = read_to_string(path);

    let mut word_count: u128 = 0;
    let mut in_word: bool = false;

    for char in content.expect("Can't read").chars() {
        if char.is_whitespace() {
            in_word = false;
        } else if !in_word {
            word_count += 1;
            in_word = true;
        }
    }
    word_count
}

pub fn count_bytes(path: &str) -> u128 {
    let metadata = metadata(path);

    metadata
        .expect("Unable to read the file contents!!")
        .len()
        .into()
}

pub fn count_lines(path: &str) -> u32 {
    let file: File = File::open(path).expect("Unable to open file!!");

    let buffreader: BufReader<File> = BufReader::new(file);

    let mut cnt: u32 = 0;

    for _ in buffreader.lines() {
        cnt = cnt + 1;
    }
    cnt
}

pub fn do_all(path: &str) {
    println!(
        "{} {} {} {}",
        count_lines(path),
        count_words(path),
        count_bytes(path),
        path
    )
}

fn main() {
    let args: Vec<String> = args().collect();

    let option: &str;
    let path: &str;

    // println!("{}", args.len());

    if args.len() == 1 || &args[1] == "-h" || &args[1] == "--help" {
        println!("Usage: ccwc [OPTION]... [FILE]...");
        println!("Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified.");
        println!("With no FILE, or when FILE is -, read standard input.");

        println!("\nOptions:");
        println!("-c, --bytes     print the byte counts");
        println!("-m              print the character counts");
        println!("-w, --words     print the word counts");
        println!("-l, --lines     print the newline counts");

        return;
    } else if args.len() == 2 {
        option = "blah";
        path = &args[1];
    } else {
        option = &args[1];
        path = &args[2];
    }

    // let path: String = "/home/avik/Desktop/practice/programming-work/codingchallenges/rust/Build_Your_Own_wc_Tool/poem.txt".to_string();

    match option.as_ref() {
        "-l" => println!("{} {}", count_lines(path), path),
        "-c" => println!("{} {}", count_bytes(path), path),
        "-w" => println!("{} {}", count_words(path), path),
        "-m" => println!("{} {}", count_multi_bytes(path), path),
        _ => do_all(path),
    }
}
