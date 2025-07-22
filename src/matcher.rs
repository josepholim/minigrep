use regex::Regex;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use crate::args::Args;

pub enum MatcherKind {
    Exact(String),
    CaseInsensitive(String),
    Regex(Regex),
    Fuzzy(String, SkimMatcherV2, i64),
}

pub struct Matcher {
    kind: MatcherKind,
    invert: bool,
}

impl Matcher {
    /// Create a Matcher from CLI arguments, capturing both strategy and inversion.
    pub fn new(args: &Args) -> Self {
        let kind = if args.fuzzy {
            MatcherKind::Fuzzy(
                args.query.clone(),
                SkimMatcherV2::default(),
                args.fuzzy_threshold,
            )
        } else if args.regex {
            MatcherKind::Regex(Regex::new(&args.query).unwrap())
        } else if args.ignore_case {
            MatcherKind::CaseInsensitive(args.query.to_lowercase())
        } else {
            MatcherKind::Exact(args.query.clone())
        };
        Matcher { kind, invert: args.invert }
    }

    /// Test whether the given line matches according to the chosen strategy,
    /// applying inversion if requested.
    pub fn is_match(&self, line: &str) -> bool {
        let hit = match &self.kind {
            MatcherKind::Exact(q) => line.contains(q),
            MatcherKind::CaseInsensitive(q) => line.to_lowercase().contains(q),
            MatcherKind::Regex(re) => re.is_match(line),
            MatcherKind::Fuzzy(query, matcher, threshold) => {
                matcher
                    .fuzzy_match(line, query)
                    .filter(|&score| score >= *threshold)
                    .is_some()
            }
        };
        if self.invert { !hit } else { hit }
    }
}
