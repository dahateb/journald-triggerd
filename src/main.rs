use async_trait::async_trait;
use handler::JournalEventHandler;
use matcher::RuleMatcher;
use parser::JournalParser;
use clap::Parser;

mod handler;
mod matcher;
mod parser;

#[async_trait]
pub trait EventHandler {
    type Event;

    async fn handle(&self, event: &Self::Event);
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// config file with rules
   #[arg(short, long, default_value = "rule.toml")]
   config_file: String,

}


#[tokio::main]
async fn main() {
    let args = Args::parse();

    let rule_file = &args.config_file;
    let matcher = RuleMatcher::new(rule_file);

    let parser = JournalParser::new(Box::new(JournalEventHandler::new(matcher)));
    parser.start_parser().await;
}
