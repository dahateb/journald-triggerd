use async_trait::async_trait;
use handler::JournalEventHandler;
use parser::JournalParser;
use matcher::RuleMatcher;

mod handler;
mod parser;
mod matcher;

#[async_trait]
pub trait EventHandler {
    type Event;

    async fn handle(&self, event: &Self::Event);
}

#[tokio::main]
async fn main() {
    let rule_file = "rule.toml";
    let matcher = RuleMatcher::new(rule_file);

    let parser = JournalParser::new(Box::new(JournalEventHandler {matcher: matcher}));
    parser.start_parser().await;
}
