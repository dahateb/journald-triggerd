use async_trait::async_trait;
use handler::JournalEventHandler;
use matcher::RuleMatcher;
use parser::JournalParser;

mod handler;
mod matcher;
mod parser;

#[async_trait]
pub trait EventHandler {
    type Event;

    async fn handle(&self, event: &Self::Event);
}

#[tokio::main]
async fn main() {
    let rule_file = "rule.toml";
    let matcher = RuleMatcher::new(rule_file);

    let parser = JournalParser::new(Box::new(JournalEventHandler::new(matcher)));
    parser.start_parser().await;
}
