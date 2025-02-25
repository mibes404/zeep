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

pub fn write_boilerplate_check_restrictions<W, S>(writer: &mut W, rust_name: S) -> WriterResult<()>
where
    W: io::Write,
    S: std::fmt::Display,
{
    writeln!(writer, "impl restrictions::CheckRestrictions for {rust_name} {{")?;
    writeln!(
        writer,
        "  fn check_restrictions(&self, _restrictions: Option<restrictions::Restrictions>) -> error::SoapResult<()> {{"
    )?;
    writeln!(writer, "     Ok(())")?;
    writeln!(writer, "  }}")?;
    writeln!(writer, "}}")?;
    Ok(())
}
