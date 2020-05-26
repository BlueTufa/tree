extern crate regex;
use std::fs;
use std::io;
use std::io::Write;
use std::io::stderr;
use std::env;
use regex::Regex;

fn filter_scan(input: &str, filter: &str) -> bool {
  let reg_ex: Regex = Regex::new(filter).unwrap();
  reg_ex.is_match(input)
}

fn main() -> io::Result<()> {
  let mut filter: Option<String> = None;
  for arg in std::env::args().skip(1) {
    filter = Some(arg);
    break;
  }
  let cwd = get_path().unwrap().work_dir;
  let options = Options::new( filter);
  recurse(&cwd, &options)
}

fn recurse(start: &str, options: &Options) -> io::Result<()> {
    let items: Vec<_> = fs::read_dir(start)?.map(|item| item.unwrap().path()).collect();
    for item in items {
      match (item.is_dir(), item.is_file()) {
        (true, false) => {
          let dir = item.to_str().unwrap();
          match recurse(&dir, &options) {
            Err(_) => {
              writeln!(stderr(), "ERROR: Unable to read path {}", dir)?;
            },
            _ => {}
          }
        },
        (false, true) => {
          let full_path = item.to_str();
          match &options.filters {
            Some(f) => {
              if filter_scan(full_path.unwrap(), f) {
                println!("{}", full_path.unwrap());
              }
            },
            _ => { 
              println!("{}", full_path.unwrap()); 
            }
          }
        },
        (_, _) => {
          writeln!(stderr(), "ERROR: Unable to read path {}", item.to_str().unwrap())?;
        }
      }
    }
  Ok(())
}

pub struct Path {
  work_dir: String
}

fn get_path() -> std::io::Result<Path> {
  let buf = env::current_dir()?;
  let work_dir =  buf.display().to_string();
  Ok(Path{ work_dir })
}

pub struct Options {
  filters: Option<String>
}

impl Options {
  pub fn new(filters: Option<String>) -> Options {
    Options {
      filters: filters
    }
  }
}