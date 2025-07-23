use regex::Regex;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use crate::args::Args;

pub enum MatcherKind {
    Exact(Vec<String>),
    CaseInsensitive(Vec<String>),
    Regex(Vec<Regex>),
    Fuzzy(Vec<String>, SkimMatcherV2, i64),
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
            let mut regexes = Vec::with_capacity(args.query.len());
            for q in &args.query {
                let re = Regex::new(q).unwrap();
                regexes.push(re);
            }
            MatcherKind::Regex(regexes)
        } else if args.ignore_case {
            let ci_query = args.query.iter().map(|q| q.to_lowercase()).collect();
            MatcherKind::CaseInsensitive(ci_query)
        } else {
            MatcherKind::Exact(args.query.clone())
        };

        Matcher { kind, invert: args.invert }
    }

    /// Test whether the given line matches according to the chosen strategy,
    /// applying inversion if requested.
    pub fn is_match(&self, line: &str) -> bool {
        let hit = match &self.kind {
            MatcherKind::Exact(queries) => {
                queries.iter().any(|q| line.contains(q))
            }
            MatcherKind::CaseInsensitive(queries) => {
                queries.iter().any(|q| line.to_lowercase().contains(q))
            }
            MatcherKind::Regex(regexes) => {
                regexes.iter().any(|re| re.is_match(line))
            }
            MatcherKind::Fuzzy(queries, matcher, threshold) => {
                queries.iter().any(|query| {
                    matcher
                        .fuzzy_match(line, query)
                        .filter(|&score| score >= *threshold)
                        .is_some()
                })
            }
        };
        if self.invert { !hit } else { hit }
    }
}
