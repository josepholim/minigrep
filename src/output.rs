use crate::args::Args;

pub fn print_matches(matches: &[(usize, &str)], args: &Args) {
    if args.count {
        println!("Match count: {}", matches.len());
        return;
    }

    for &(idx, line) in matches {
        if args.show_line_numbers {
            print!("{:>6}  ", idx + 1);
        }
        println!("{}", line);
    }
}