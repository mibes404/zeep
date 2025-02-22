use crate::{error::WriterResult, model::Namespace, reader::WriteXml};
use std::{io, rc::Rc};

pub struct SoapEnvelope {
    pub target_namespaces: Vec<Rc<Namespace>>,
}

impl<W> WriteXml<W> for SoapEnvelope
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        writeln!(writer, "<soap:Envelope")?;
        writeln!(writer, "    xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\"")?;
        writeln!(writer, "    xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"")?;
        writeln!(writer, "    xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">")?;
        writeln!(writer, "    <soap:Body>")?;
        writeln!(writer, "    </soap:Body>")?;
        writeln!(writer, "</soap:Envelope>")?;

        Ok(())
    }
}
