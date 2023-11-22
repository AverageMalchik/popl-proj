#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrepOptions {
  pub search: String,
  pub file_path: String,
  pub case_sensitive: bool,
  pub count: bool,
}

const REQUIRED_ARGS_COUNT: usize = 3;

pub struct GrepOptionsBuilder;

impl GrepOptionsBuilder {
  pub fn parse(args: Vec<String>) -> Result<GrepOptions, String> {
    if args.len() < REQUIRED_ARGS_COUNT {
      return Err(format!(
        "Expected at least {} arguments, got {}.\nUsage: ",
        REQUIRED_ARGS_COUNT,
        args.len()
      ));
    }

    let mut args = args.iter();
    args.next(); // Skip the first argument.

    let options = GrepOptions {
      search: match args.next() {
        Some(arg) => arg.clone(),
        None => String::new(),
      },
      file_path: match args.next() {
        Some(arg) => arg.clone(),
        None => String::new(),
      },
      case_sensitive: args.next().is_some(), // If 3rd arg exists, then true.
      count: args.next().is_some(),
    };

    Ok(options)
  }
}