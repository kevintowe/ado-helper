use std::path::PathBuf;
// use std::fs;
// use std::process::Command;
use dirs::home_dir;
use std::env;
// use reqwest::blocking::Client;
// use reqwest::header;

pub fn get_pat_file_path() -> PathBuf {
    let mut path = home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".ado_helper");
    path.push("pat.txt");
    path
}

pub fn get_org_file_path() -> PathBuf {
    let mut path = home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".ado_helper");
    path.push("org.txt");
    path
}

// Handle cwd resolution and validation
pub fn resolve_and_validate_cwd(cwd: Option<PathBuf>) -> PathBuf {
    let cwd_path =
        cwd.unwrap_or_else(|| env::current_dir().expect("Failed to get current working directory"));

    if !cwd_path.exists() || !cwd_path.is_dir() {
        eprintln!(
            "Error: The specified working directory does not exist or is not a directory: {:?}",
            cwd_path
        );
        std::process::exit(1);
    }

    println!("Using working directory: {:?}", cwd_path);

    cwd_path
}

// pub fn create_authenticated_ado_request() -> Result<Client, String> {
//   let token = read_pat().map_err(|e| e.to_string())?;
//   let client = Client::new();

//   Ok(client
//     .get()
//     .header(header::AUTHORIZATION, format!("Basic {}", base64::encode(format!("{}:", token.trim()))))
//     .build()
//     .expect("Failed to build http client."))
// }

// pub fn clone_repo(repo_url: &str, install_path: &PathBuf) {
//   let status = Command::new("git")
//     .arg("clone")
//     .arg(repo_url)
//     .arg(install_path)
//     .status()
//     .expect("Failed to execute git");

//   if !status.success() {
//     eprintln!("Failed to clone repo: {}", repo_url);
//   }
// }

// fn read_pat() -> Result<String, std::io::Error> {
//   let path = get_pat_file_path();
//   fs::read_to_string(path)
// }
