use super::EventHandler;
use async_trait::async_trait;
use journald::JournalEntry;
use super::matcher::RuleMatcher;

pub struct JournalEventHandler {
    pub matcher: RuleMatcher
}

#[async_trait]
impl EventHandler for JournalEventHandler {
    type Event = JournalEntry;

    async fn handle(&self, event: &Self::Event) {
        let fields = event.get_fields();
        let unit = if fields.get("_SYSTEMD_UNIT").is_some() {
            fields.get("_SYSTEMD_UNIT").unwrap()
        } else if fields.get("_SYSTEMD_USER_UNIT").is_some() {
            fields.get("_SYSTEMD_USER_UNIT").unwrap()
        } else {
            "unknown"
        };
        let message = event.get_message().expect("message should have been there");
        println!(
            "Unit: {} : message {}",
            unit,
            message
        );
        let rule_match = self.matcher.matches(message.to_string());
        if rule_match.is_some() {
            println!("Triggered: {}", rule_match.unwrap().trigger);
        }
    }
}
