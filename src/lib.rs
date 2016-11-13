/// slog formatter for MessagePack using the rmp-serde library.
///
/// Implementation translated straight from the JSON formatter. Not
/// sure if good.
extern crate slog;
extern crate slog_stream;
extern crate slog_serde;
extern crate rmp_serde;

use std::io;
use slog_serde::SerdeSerializer;
use slog::{Record, OwnedKeyValueList};
use rmp_serde::encode::Serializer as RmpSerializer;

pub struct Format {
    values: OwnedKeyValueList
}

impl Format {
    pub fn new() -> FormatBuilder {
        FormatBuilder::new()
    }
}

pub struct FormatBuilder {
    values: OwnedKeyValueList,
}

impl FormatBuilder {
    fn new() -> Self {
        FormatBuilder {
            values: OwnedKeyValueList::root(None)
        }
    }

    pub fn build(self) -> Format {
        Format { values: self.values }
    }
}

impl slog_stream::Format for Format {
    fn format(&self,
              io: &mut io::Write,
              rinfo: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        let serializer = RmpSerializer::new(io);
        let len = self.values.iter().count()
            + logger_values.iter().count()
            + rinfo.values().iter().count();
        let mut serializer = SerdeSerializer::start(serializer, Some(len))?;

        for (ref k, ref v) in self.values.iter() {
            v.serialize(rinfo, k, &mut serializer)?;
        }

        for (ref k, ref v) in logger_values.iter() {
            v.serialize(rinfo, k, &mut serializer)?;
        }

        for &(ref k, ref v) in rinfo.values().iter() {
            v.serialize(rinfo, k, &mut serializer)?;
        }

        let (_serializer, res) = serializer.end();

        let _ = res?;

        Ok(())
    }
}

pub fn new() -> FormatBuilder {
    Format::new()
}
