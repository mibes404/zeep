use crate::{error::WriterResult, reader::WriteXml};
use const_format::{concatc, formatc};
use std::io;

pub struct FileHeader;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION: &str = formatc!("//! version: {PKG_VERSION}");

impl<W> WriteXml<W> for FileHeader
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        const HEADER: &str = concatc!(
            r"//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
",
            VERSION,
            r#"
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]

use std::io::{Read, Write};
use std::rc::Rc;
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
"#
        );

        write!(writer, "{HEADER}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_write_header() {
        let mut buffer = vec![];
        let header = FileHeader;
        header.write_xml(&mut buffer).unwrap();
        let str_buffer = String::from_utf8(buffer).unwrap();
        assert!(str_buffer.contains("THIS IS A GENERATED FILE!"));
        assert!(str_buffer.contains(PKG_VERSION));
        assert!(str_buffer.contains(VERSION));
    }
}
