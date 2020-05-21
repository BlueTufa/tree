pub struct Tree<'args> {
  pub args: Vec<&'args OsStr>
}
pub struct Filters {
  work_dir: String,
  filters: bool,
  regexes: Vec<&'args OsStr>
}

impl<'args> Tree {
  pub fn from_args<I>(args: I) -> Result<Tree<'args>>
  where I: Iterator<Item=&'args OsString> {
    
  }
}
