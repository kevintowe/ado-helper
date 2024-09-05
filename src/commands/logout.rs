use std::fs;
use crate::commands::util::get_pat_file_path;

pub fn run() {
  let path = get_pat_file_path();

  // Check if the PAT file exists
  if path.exists() {
    // Attempt to delete the file
    match fs::remove_file(&path) {
      Ok(_) => {
        println!("PAT file deleted sucessfully.");
      }
      Err(e) => {
        eprintln!("Failed to delete PAT file: {}", e);
      }
    }
  } else {
    println!("No PAT file found, user not logged in.");
  }
}