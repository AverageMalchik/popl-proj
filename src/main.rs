use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::env;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let mut current_dir = env::current_dir()?;
    let mut selected_entry = 0;

    loop {
        print_directory_contents(&current_dir, selected_entry)?;

        // Read and process user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "q" => break, // Quit the file explorer
            "j" => {
                // Move selection down
                selected_entry = (selected_entry + 1).min(get_entries(&current_dir)?.len() - 1);
            }
            "k" => {
                // Move selection up
                selected_entry = selected_entry.saturating_sub(1);
            }
            "o" => {
                // Open selected entry (file or directory)
                if let Some(entry_path) = get_selected_entry(&current_dir, selected_entry) {
                    let entry_path = entry_path.as_path();
                    if entry_path.is_dir() {
                        current_dir = PathBuf::from(entry_path);
                        selected_entry = 0;
                    } else {
                        open_file(entry_path)?;
                    }
                }
            }
            "b" => {
                // Go back to the parent directory
                if let Some(parent) = current_dir.parent() {
                    current_dir = PathBuf::from(parent);
                    selected_entry = 0;
                }
            }
            "s" => {
                // Search for a file anywhere in the computer
                println!("Enter the file name to search for: ");
                let search_query = get_user_input();

                let root_directories = vec!["D:\\"];

                let mut search_results = Vec::new();

                for root_dir in root_directories {
                    let dir_results = search_file_in_directory(root_dir, &search_query);
                    search_results.extend(dir_results);
                }

                if search_results.is_empty() {
                    println!("No matching files found.");
                } else {
                    println!("Matching files:");
                    for result in &search_results {
                        println!("{}", result.display());
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn print_directory_contents(current_dir: &Path, selected_entry: usize) -> io::Result<()> {
    println!("Current directory: {}", current_dir.display());

    for (i, entry) in get_entries(current_dir)?.iter().enumerate() {
        if i == selected_entry {
            print!("> "); // Highlight the selected entry
        } else {
            print!("  ");
        }
        println!("{}", entry.file_name().to_string_lossy());
    }

    print!("(j: down, k: up, o: open, b: go back, q: quit, s: search) > ");
    io::stdout().flush()?;
    Ok(())
}

fn get_entries(current_dir: &Path) -> io::Result<Vec<fs::DirEntry>> {
    let entries: io::Result<Vec<_>> = current_dir.read_dir()?.collect();
    entries
}

fn get_selected_entry(current_dir: &Path, selected_entry: usize) -> Option<PathBuf> {
    get_entries(current_dir)
        .ok()
        .and_then(|entries| entries.get(selected_entry).map(|entry| entry.path()))
}

fn open_file(file_path: &Path) -> io::Result<()> {
    let mut file = fs::File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents for {}: \n{}", file_path.display(), contents);
    Ok(())
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn search_file_in_directory(root_dir: &str, search_query: &str) -> Vec<std::path::PathBuf> {
    let mut results = Vec::new();

    for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.to_lowercase().contains(&search_query.to_lowercase()) {
                results.push(entry.path().to_path_buf());
            }
        }
    }

    results
}

