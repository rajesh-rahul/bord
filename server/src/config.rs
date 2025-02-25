use hashbrown::HashMap;
use regex::Regex;

#[derive(Debug)]
pub struct BordConfig {
    pub match_patterns: HashMap<String, MatchPattern>,
}

#[derive(Debug)]
pub struct MatchPattern {
    re: Regex,
    named_groups: Vec<String>,
}

impl Default for BordConfig {
    fn default() -> Self {
        Self {

            match_patterns: HashMap::from([
                (
                    "rust".to_string(), 
                    MatchPattern {
                        re: Regex::new(r#"sql!\(\s*(?:"?(?<n1>(?:\\.|[^"\\])*)"|r#"(?<n2>(?:\\.|[^"\\])*)"\#)\s*\)"#).unwrap(), 
                        named_groups: vec!["n1".into(), "n2".into()] 
                    }
                )
            ])
        }
    }
}

impl MatchPattern {
    pub fn match_on_haystack<'a, 'b: 'a>(
        &'b self,
        haystack: &'a str,
    ) -> impl Iterator<Item = regex::Match<'a>> {
        self.re
            .captures_iter(haystack)
            .flat_map(|cap| self.named_groups.iter().flat_map(move |nm| cap.name(nm)))
    }
}
