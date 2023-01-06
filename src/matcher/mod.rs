use serde_derive::Deserialize;
use toml;
use std::fs;

#[derive(Deserialize)]
struct Config {
    rule: Vec<Rule>
}

#[derive(Deserialize)]
struct Rule {
    filter: String,
    trigger: String
}


pub struct RuleMatcher {
    config: Config
}

pub struct RuleMatch {
    pub filter: String,
    pub trigger: String
}

impl RuleMatcher {
    
    pub fn new(file_path: &str) -> RuleMatcher {
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
        let config: Config = toml::from_str(contents.as_str()).expect("Error parsing toml file");
        RuleMatcher { config: config }
    }

    pub fn matches(&self, value: String) -> Option<RuleMatch> {
        for rule in &self.config.rule {
            if value == rule.filter {
                return Some(RuleMatch { filter: rule.filter.clone(), trigger: rule.trigger.clone() });
            }
        }

        None
    }

}