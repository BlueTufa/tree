extern crate regex;
#[macro_use]
extern crate lazy_static;
use std::fs;
use std::io;
use std::env;
use regex::RegexSet;
use std::ffi::OsString;
use std::env::args_os;

fn filter_scan(input: &str) -> bool {
  lazy_static! {
    static ref RE: RegexSet = RegexSet::new(&[r"\.git"]).unwrap();
  }
  let matches: Vec<_> = RE.matches(input).into_iter().collect();
  !matches.is_empty()
}

fn main() -> io::Result<()> {
  let args: Vec<OsString> = args_os().skip(1).collect();
  for arg in args {
    println!("{:?}", arg);
  }
  let cwd = get_path().unwrap().work_dir;
  recurse(&cwd)
}

fn recurse(start: &str) -> io::Result<()> {
    let items: Vec<_> = fs::read_dir(start)?.map(|item| item.unwrap().path()).collect();
    for item in items {
      if item.is_dir() {
        let dir = item.to_str().unwrap();
        recurse(&dir).expect("Could not traverse directory.");
      } else {
        let full_path = item.to_str();
        if filter_scan(full_path.unwrap()) {
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
