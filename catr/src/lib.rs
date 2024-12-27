use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => {
                let lines = read_lines(open(&filename)?)?;
                eprintln!("Opened {}: {}", filename, lines.join("\n"));
            }
        }
    }
    Ok(())
}

pub fn read_lines<T: BufRead>(reader: T) -> MyResult<Vec<String>> {
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

// Store the heap memory pointer to the stack memory, because the dyn Error does not know size of retrun value
// https://doc.rust-jp.rs/book-ja/ch15-01-box.html#%E3%83%92%E3%83%BC%E3%83%97%E3%81%AE%E3%83%87%E3%83%BC%E3%82%BF%E3%82%92%E6%8C%87%E3%81%99boxt%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%99%E3%82%8B
pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("dak2")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input File(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines")
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blnak lines")
                .takes_value(false),
        )
        .get_matches();

      Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
      })
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}
