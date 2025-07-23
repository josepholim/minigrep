use std::collections::HashSet;
use crate::args::Args;
use colored::Colorize;
use regex::RegexBuilder;

pub fn print_matches(matches: &[(usize, &str)], args: &Args) {
    if args.count {
        println!("Match count: {}", matches.len());
        return;
    }

    for &(idx, line) in matches {
        let line_output = highlight(line, args);
        
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

        let line_output = highlight(line, args);

        if args.show_line_numbers {
            println!("{} {:>4}: {}", prefix, i + 1, line_output);
        } else {
            println!("{} {}", prefix, line_output);
        }

        prev = Some(i);
    }
}

pub fn highlight(line: &str, args: &Args) -> String {
    if !args.color {
        return line.to_string();
    }

    let mut spans: Vec<(usize, usize)> = Vec::new();
    let hay = if args.ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };

    if args.regex {
        for q in &args.query {
            if let Ok(re) = RegexBuilder::new(q)
                .case_insensitive(args.ignore_case)
                .build()
            {
                for mat in re.find_iter(line) {
                    spans.push((mat.start(), mat.end()));
                }
            }
        }
    } else {
        for q in &args.query {
            let q_cmp = if args.ignore_case {
                q.to_lowercase()
            } else {
                q.clone()
            };
            let mut search_start = 0;
            while let Some(pos) = hay[search_start..].find(&q_cmp) {
                let s = search_start + pos;
                spans.push((s, s + q_cmp.len()));
                search_start = s + q_cmp.len();
            }
        }
    }

    // Sort spans and merge overlapping/adjacent ones
    spans.sort_unstable_by_key(|&(s, _)| s);
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (s, e) in spans {
        if let Some(last) = merged.last_mut() {
            if s <= last.1 {
                last.1 = last.1.max(e);
                continue;
            }
        }
        merged.push((s, e));
    }

    // Rebuild the line with ANSI coloring for each span
    let mut result = String::new();
    let mut last_end = 0;
    for (s, e) in merged {
        result.push_str(&line[last_end..s]);
        result.push_str(&line[s..e].red().bold().to_string());
        last_end = e;
    }
    result.push_str(&line[last_end..]);

    result
}
