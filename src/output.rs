use std::collections::HashSet;
use crate::args::Args;
use colored::Colorize;

pub fn print_matches(matches: &[(usize, &str)], args: &Args) {
    if args.count {
        println!("Match count: {}", matches.len());
        return;
    }

    for &(idx, line) in matches {
        let line_output = if args.color {
            line.replace(
                &args.query,
                &args.query.red().bold().to_string(),
            )
        } else {
            line.to_string()
        };
        
        if args.show_line_numbers {
            print!("{:>6}  ", idx + 1);
        }
        println!("{}", line_output);
    }
}

pub fn print_matches_with_context(
    lines: &[&str], 
    match_idxs: &[usize], 
    context_set: &HashSet<usize>,
    args: &Args
) {
    let mut prev: Option<usize> = None;
    let match_set: HashSet<usize> = match_idxs.iter().copied().collect();

    for (i, &line) in lines.iter().enumerate() {
        if !context_set.contains(&i) {
            continue;
        }

        // Insert separator if there’s a gap
        if let Some(last) = prev {
            if i > last + 1 {
                println!("--");
            }
        }

        // Choose prefix: '>' for match, '|' for context
        let prefix = if match_set.contains(&i) { '>' } else { '|' };

        let line_output = if args.color {
            line.replace(
                &args.query,
                &args.query.red().bold().to_string(),
            )
        } else {
            line.to_string()
        };

        if args.show_line_numbers {
            println!("{} {:>4}: {}", prefix, i + 1, line_output);
        } else {
            println!("{} {}", prefix, line_output);
        }

        prev = Some(i);
    }
}