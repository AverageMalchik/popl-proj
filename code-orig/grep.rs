use std::{error::Error, fs};

use crate::grep_command_builder::GrepOptions;

use r3bl_rs_utils::{style_primary, style_prompt};

pub fn grep(options: GrepOptions) -> Result<(), Box<dyn Error>> {
    println!(
      "{}: search for '{}' in '{}' w/ {} {}",
      style_prompt("DEBUG"),
      options.search,
      options.file_path,      
      match options.case_sensitive {
        true => "case sensitive",
        false => "case insensitive",
      },
      match options.count {
          true => "only count",
          false => "",
      }
    );

    let mut counter = 0;

    let content = fs::read_to_string(options.file_path)?;
    let filtered_content = content
        .lines()
        .filter(|line| {
            let contains = if options.case_sensitive {
                line.contains(&options.search)
            } else {
                line.to_lowercase().contains(&options.search.to_lowercase())
            };
            if contains {
                counter += 1;
            }
            contains
        })
        .map(|line| {
            let from = &options.search;
            let to = format!("{}", style_primary(&options.search));
            line.replace(from, &to)
        })
        .collect::<Vec<String>>();
    if options.count {
        println!("{}", counter);
    } else {
        println!("{}", filtered_content.join("\n"));
    }

    Ok(())
}
