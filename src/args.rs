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

    /// Invert match: show lines that *do not* match
    #[arg(short = 'v', long, action = ArgAction::SetTrue)]
    pub invert: bool,

    /// Number of context lines to show before and after each match
    #[arg(short = 'C', long, default_value_t = 0, value_name = "NUM")]
    pub context: usize,

    /// Number of lines **after** each match to include
    #[arg(short = 'A', long, default_value_t = 0, value_name = "NUM")]
    pub context_after: usize,

    /// Number of lines **before** each match to include
    #[arg(short = 'B', long, default_value_t = 0, value_name = "NUM")]
    pub context_before: usize,

    /// Treat query as a regular expression
    #[arg(short = 'E', long, action = ArgAction::SetTrue)]
    pub regex: bool,

    /// Recurse into directories
    #[arg(short = 'R', long, action = ArgAction::SetTrue)]
    pub recursive: bool,

    /// Path to an ignore file to skip files
    #[arg(long, default_value = ".gitignore", value_name = "FILE")]
    pub ignore_file: String,

    /// Colorize matching text
    #[arg(long, action = ArgAction::SetTrue)]
    pub color: bool,

    /// Use fuzzy matching instead of exact substring/regex
    #[arg(long, action = ArgAction::SetTrue)]
    pub fuzzy: bool,

    /// Minimum fuzzy score to accept (higher implies stricter)
    #[arg(long, default_value_t = 1, value_name = "SCORE")]
    pub fuzzy_threshold: i64,
}
