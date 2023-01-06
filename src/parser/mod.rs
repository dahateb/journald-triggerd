use async_stream::try_stream;
use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::StreamExt;
use journald::reader::*;
use journald::Error;
use journald::JournalEntry;

use crate::EventHandler;

struct JournalStreamBuilder<'a> {
    pub reader: &'a mut JournalReader,
}

impl<'a> JournalStreamBuilder<'a> {
    pub fn new(reader: &'a mut JournalReader) -> Result<Self, Error> {
        reader.seek(JournalSeek::Tail).expect("journal seek failed");
        Ok(Self { reader })
    }

    fn next_wait(&mut self) -> Result<Option<JournalEntry>, Error> {
        let ret = self.reader.next_entry();

        if ret.is_ok() && ret.as_ref().unwrap().is_none() {
            let wakeup = self.reader.wait()?;
            if wakeup != WakeupType::NOP {
                return self.next_wait();
            }
            //log::trace!("got WakeupType '{:?}' from systemd in BlockingIter", wakeup);
            return self.reader.next_entry();
        }

        ret
    }

    pub fn get_stream(
        &'a mut self,
    ) -> impl Stream<Item = Result<Option<JournalEntry>, Error>> + 'a {
        try_stream! {
            loop {
                yield self.next_wait().unwrap();
            }
        }
    }
}

pub struct JournalParser {
    event_handler: Box<dyn EventHandler<Event = JournalEntry>>,
}

impl JournalParser {
    pub fn new(event_handler: Box<dyn EventHandler<Event = JournalEntry>>) -> JournalParser {
        JournalParser {
            event_handler: event_handler,
        }
    }

    pub async fn start_parser(&self) {
        let mut journal =
            JournalReader::open(&JournalReaderConfig::default()).expect("journal open failed");

        let mut builder = JournalStreamBuilder::new(&mut journal).expect("msg");
        let stream = builder.get_stream();
        pin_mut!(stream); // needed for iteration

        while let Some(value) = stream.next().await {
            if value.as_ref().unwrap().is_some() {
                self.event_handler.handle(&value.unwrap().unwrap()).await;   
            }                   
        }
    }
}
