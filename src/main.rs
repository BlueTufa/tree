use std::fs;
use std::io;

fn main() -> io::Result<()> {
  recurse("/Users/badger/src")
}

fn recurse(start: &str) -> io::Result<()> {
    let items: Vec<_> = fs::read_dir(start)?.map(|item| item.unwrap().path()).collect();
    for item in items {
      if item.is_dir() {
        let dir = item.to_str().unwrap();
        recurse(&dir);
      } else {
        let full_path = item.to_str();
        println!("{}", full_path.unwrap());
      }
    }
  Ok(())
}