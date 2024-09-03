use log::{{info, warn}};
use anyhow::{{Context, Result}};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Cli starting up");   
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
	.with_context(|| format!("could not read file `{}`", args.path.display()))?;

   grss::find_matches(&content, &args.pattern, &mut std::io::stdout());
 
    Ok(()) 
}
