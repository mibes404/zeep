use crate::{error::WriterResult, model::structures::restrictions::Restrictions, reader::WriteXml};
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

pub fn write_check_restrictions_header<W>(
    writer: &mut W,
    rust_name: &str,
    restrictions: Option<&Restrictions>,
) -> WriterResult<()>
where
    W: io::Write,
{
    writeln!(writer, "impl restrictions::CheckRestrictions for {rust_name} {{")?;
    if let Some(restrictions) = restrictions {
        writeln!(
            writer,
            "  fn check_restrictions(&self, _restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()>  {{"
        )?;

        writeln!(writer, "        let restrictions = Some(")?;
        restrictions.write_xml(writer)?;
        writeln!(writer, ");")?;
    } else {
        writeln!(
            writer,
            "  fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()>  {{"
        )?;
    }

    Ok(())
}

pub fn write_check_restrictions_footer<W>(writer: &mut W) -> WriterResult<()>
where
    W: io::Write,
{
    writeln!(writer, "  }}")?;
    writeln!(writer, "}}")?;
    Ok(())
}
