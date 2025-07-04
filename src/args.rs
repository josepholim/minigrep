use clap::{Parser, ArgAction};

/// minigrep: search for patterns in files
#[derive(Parser, Debug)]
#[command(
    name = "minigrep",
    version,
    about = "A simple grep tool in Rust",
    long_about = None
)]
pub struct Args {
    /// The pattern to look for
    #[arg(short, long, value_name = "PATTERN")]
    pub query: String,

    /// File or directory to search (use "-" or omit for stdin)
    #[arg(short, long, default_value = "-", value_name = "PATH")]
    pub path: String,

    /// Case-insensitive matching
    #[arg(short = 'i', long, action = ArgAction::SetTrue)]
    pub ignore_case: bool,

    /// Show line numbers for each match
    #[arg(short = 'n', long, action = ArgAction::SetTrue)]
    pub show_line_numbers: bool,

    /// Only print a count of matching lines per file
    #[arg(short = 'c', long, action = ArgAction::SetTrue)]
    pub count: bool,

    /// Number of context lines to show before and after each match
    #[arg(short = 'C', long, default_value_t = 0, value_name = "NUM")]
    pub context: usize,

    /// Treat query as a regular expression
    #[arg(short = 'E', long, action = ArgAction::SetTrue)]
    pub regex: bool,
}