extern crate regex;
use std::fs;
use std::io;
use std::env;
use regex::Regex;

fn filter_scan(input: &str, filter: &str) -> bool {
  let reg_ex: Regex = Regex::new(filter).unwrap();
  reg_ex.is_match(input)
}

fn main() -> io::Result<()> {

  let mut filters = Vec::new();
  for arg  in std::env::args().skip(1) {
    filters.push(arg)
  }
  let cwd = get_path().unwrap().work_dir;
  recurse(&cwd, &filters.get(0).unwrap())
}

fn recurse(start: &str, filter: &str) -> io::Result<()> {
    let items: Vec<_> = fs::read_dir(start)?.map(|item| item.unwrap().path()).collect();
    for item in items {
      if item.is_dir() {
        let dir = item.to_str().unwrap();
        recurse(&dir, &filter).expect("Could not traverse directory.");
      } else {
        let full_path = item.to_str();
        if filter_scan(full_path.unwrap(), filter) {
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
