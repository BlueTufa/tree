use std::fs;
use std::io;
use std::env;

fn main() -> io::Result<()> {
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
        println!("{}", full_path.unwrap());
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
