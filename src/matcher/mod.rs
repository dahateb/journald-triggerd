use serde_derive::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
struct Config {
    rule: Vec<Rule>,
}

#[derive(Deserialize)]
struct Rule {
    filter: String,
    trigger: String,
}

pub struct RuleMatcher {
    config: Config,
}

pub struct RuleMatch {
    pub filter: String,
    pub trigger: String,
}

impl RuleMatcher {
    pub fn new(file_path: &str) -> RuleMatcher {
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let config: Config = toml::from_str(contents.as_str()).expect("Error parsing toml file");
        RuleMatcher { config: config }
    }

    pub fn matches(&self, value: String) -> Option<Vec<RuleMatch>> {
        let mut rule_matches = Vec::new();
        for rule in &self.config.rule {
            if value == rule.filter {
                rule_matches.push(RuleMatch {
                    filter: rule.filter.clone(),
                    trigger: rule.trigger.clone(),
                });
            }
        }

        if rule_matches.len() > 0 {
            return Some(rule_matches);
        }

        None
    }
}
