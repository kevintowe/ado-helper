use std::fs;
use std::io::Write;
use rpassword::prompt_password;
use crate::commands::util::get_pat_file_path;

pub fn run() {
    // Prompt the user for their Azure DevOps PAT
    let token = prompt_password("Enter your Azure DevOps PAT: ").unwrap();

    // Determine the location to persist the PAT
    let path = get_pat_file_path();

    // Create the directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Persist the PAT
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(token.as_bytes()).unwrap();
    
    println!("PAT saved to {}", path.display());
}