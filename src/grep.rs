use std::{error::Error, fs};

use crate::grep_command_builder::GrepOptions;

use r3bl_rs_utils::{style_primary, style_prompt};

pub fn grep(options: GrepOptions) -> Result<(), Box<dyn Error>> {
  print!(
    "{}: search for '{}' in '{}' w/ flags ",
    style_prompt("DEBUG"),
    options.search,
    options.file_path
  );

  for flag in options.fl.iter() {
    print!();
  }

  println!();

  let content = fs::read_to_string(options.file_path)?;
  let filtered_content = content
    .lines()
    .filter(|line| {
      if options.case_sensitive {
        line.contains(&options.search)
      } else {
        line.to_lowercase().contains(&options.search.to_lowercase())
      }
    })
    .map(|line| {
      let from = &options.search;
      let to = format!("{}", style_primary(&options.search));
      line.replace(from, &to)
    })
    .collect::<Vec<String>>();
  println!("{}", filtered_content.join("\n"));

  Ok(())
}