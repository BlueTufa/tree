pub struct Tree<'args> {
  pub args: Vec<&'args OsStr>
}
pub struct Filters {
  work_dir: String,
  regex: Option<String>
}

impl<'args> Tree {
  pub fn from_args<I>(args: I) -> Result<Tree<'args>>
  where I: Iterator<Item=&'args OsString> {

  }
}
