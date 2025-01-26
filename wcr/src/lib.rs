use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  for filename in &config.files {
    match open(filename) {
      Err(err) => eprintln!("Failed to open {}: {}", filename, err),
      Ok(file) => {
        if let Ok(info) = count(file) {
          println!("{:>8}{:>8}{:>8} {}",
            info.num_lines,
            info.num_words,
            info.num_bytes,
            filename
          );
        }
      }
    }
  }

  Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
      "-" => Ok(Box::new(BufReader::new(io::stdin()))),
      _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
  let mut num_lines = 0;
  let mut num_words = 0;
  let mut num_chars = 0;
  let mut num_bytes = 0;
  let mut line = String::new();

  loop {
    let line_bytes = file.read_line(&mut line)?;
    if line_bytes == 0 {
      break;
    }
    num_bytes += line_bytes;
    num_lines += 1;
    num_words += line.split_whitespace().count();
    num_chars += line.chars().count();
    line.clear();
  }

  Ok(FileInfo {
    num_lines,
    num_words,
    num_chars,
    num_bytes,
  })
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("wcr")
    .version("0.1.0")
    .author("dak2")
    .about("Rust wc")
    .arg(
      Arg::with_name("files")
        .value_name("FILES")
        .help("Input file(s) [default: -]")
        .multiple(true)
        .default_value("-"),
    )
    .arg(
      Arg::with_name("lines")
        .short("l")
        .help("Show line count")
        .takes_value(false)
    )
    .arg(
      Arg::with_name("words")
        .short("w")
        .help("Show word count")
        .takes_value(false)
    )
    .arg(
      Arg::with_name("chars")
        .short("m")
        .help("Show character count")
        .takes_value(false)
        .conflicts_with("bytes"),
    )
    .arg(
      Arg::with_name("bytes")
        .short("c")
        .help("Show byte count")
        .takes_value(false)
        .conflicts_with("chars"),
    )
    .get_matches();

  let files = matches.values_of_lossy("files").unwrap();
  let lines = matches.is_present("lines");
  let words = matches.is_present("words");
  let bytes = matches.is_present("bytes");
  let chars = matches.is_present("chars");

  gen_config(files, lines, words, bytes, chars)
}

fn gen_config(
  files: Vec<String>,
  mut lines: bool,
  mut words: bool,
  mut bytes: bool,
  chars: bool,
) -> MyResult<Config> {

  if [lines, words, bytes, chars].iter().all(|&x| !x) {
    lines = true;
    words = true;
    bytes = true;
  }

  Ok(Config {
      files,
      lines,
      words,
      bytes,
      chars,
  })
}

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  lines: bool,
  words: bool,
  bytes: bool,
  chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
  num_lines: usize,
  num_words: usize,
  num_bytes: usize,
  num_chars: usize,
}

#[cfg(test)]
mod tests {
  use super::{count, FileInfo};
  use std::io::Cursor;

  #[test]
  fn test_count() {
    let text = "I don't want the world. I just want your half.\r\n";
    let info = count(Cursor::new(text));
    assert!(info.is_ok());
    let expected = FileInfo {
      num_lines: 1,
      num_words: 10,
      num_chars: 48,
      num_bytes: 48,
    };
    assert_eq!(info.unwrap(), expected);
  }
}
