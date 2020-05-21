extern crate regex;
use std::fs;
use std::io;
use std::env;
use regex::Regex;

fn filter_scan(input: &str, filter: String) -> bool {
  let reg_ex: Regex = Regex::new(&filter).unwrap();
  reg_ex.is_match(input)
}

fn main() -> io::Result<()> {
  let mut filter: Option<String> = None;
  for arg  in std::env::args().skip(1) {
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
      if item.is_dir() {
        let dir = item.to_str().unwrap();
        recurse(&dir, &options).expect("Could not traverse directory.");
      } else {
        let full_path = item.to_str();
        if options.filters.is_some() 
          && filter_scan(full_path.unwrap(), options.filters.unwrap().clone()) {
          println!("{}", full_path.unwrap());
        } else { 
          println!("{}", full_path.unwrap());
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