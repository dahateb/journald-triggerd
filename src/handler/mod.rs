use super::matcher::RuleMatcher;
use super::EventHandler;
use async_trait::async_trait;
use journald::JournalEntry;
use std::{sync::Arc, sync::Mutex};
use tokio::process::Command;

pub struct JournalEventHandler {
    pub matcher: RuleMatcher,
    counter: Arc<Mutex<u64>>,
}

impl JournalEventHandler {
    pub fn new(matcher: RuleMatcher) -> JournalEventHandler {
        JournalEventHandler {
            matcher: matcher,
            counter: Arc::new(Mutex::new(0)),
        }
    }
}

#[async_trait]
impl EventHandler for JournalEventHandler {
    type Event = JournalEntry;

    async fn handle(&self, event: &Self::Event) {
        let counter;
        {
            *self.counter.lock().as_deref_mut().unwrap() += 1;
            counter = *self.counter.lock().unwrap();
        }

        let fields = event.get_fields();
        let unit = if fields.get("_SYSTEMD_UNIT").is_some() {
            fields.get("_SYSTEMD_UNIT").unwrap()
        } else if fields.get("_SYSTEMD_USER_UNIT").is_some() {
            fields.get("_SYSTEMD_USER_UNIT").unwrap()
        } else {
            "unknown"
        };
        let message = event.get_message().expect("message should have been there");
        println!("{} Unit: {} : message {}", counter, unit, message);
        if counter > 71 {
            if let Some(rule_matches) = self.matcher.matches(message.to_string()) {
                for rule_match in rule_matches {
                    let _handle = tokio::spawn(async {
                        println!("Triggered: {}", rule_match.trigger);
                        let _result = Command::new("sh")
                            .arg("-c")
                            .arg(rule_match.trigger)
                            .output()
                            .await;
                    });
                }
            }
        }
    }
}
