use crate::{error::WriterResult, reader::WriteXml};
use std::io;

pub struct Helpers;

impl<W> WriteXml<W> for Helpers
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        const HELPERS: &str = include_str!("helpers_content.rs");
        write!(writer, "{HELPERS}")?;
        Ok(())
    }
}
