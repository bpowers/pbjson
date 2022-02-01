//! This module contains the actual code generation logic

use std::fmt::{Display, Formatter};
use std::io::{Result, Write};

mod enumeration;
mod message;

pub use enumeration::generate_enum;
pub use message::generate_message;
use crate::message::Message;

#[derive(Debug, Clone, Copy)]
struct Indent(usize);

impl Display for Indent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.0 {
            write!(f, "    ")?;
        }
        Ok(())
    }
}

fn write_fields_array<'a, W: Write, I: Iterator<Item = &'a str>>(
    writer: &mut W,
    indent: usize,
    variants: I,
) -> Result<()> {
    writeln!(writer, "{}const FIELDS: &[&str] = &[", Indent(indent))?;
    for name in variants {
        writeln!(writer, "{}\"{}\",", Indent(indent + 1), name)?;
    }
    writeln!(writer, "{}];", Indent(indent))?;
    writeln!(writer)
}

fn write_serialize_start<W: Write>(indent: usize, rust_type: &str, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        r#"{indent}impl serde::Serialize for {rust_type} {{
{indent}    #[allow(deprecated)]
{indent}    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
{indent}    where
{indent}        S: serde::Serializer,
{indent}    {{"#,
        indent = Indent(indent),
        rust_type = rust_type
    )
}

fn write_serialize_end<W: Write>(indent: usize, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        r#"{indent}    }}
{indent}}}"#,
        indent = Indent(indent),
    )
}

fn write_deserialize_start<W: Write>(indent: usize, rust_type: &str, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        r#"{indent}impl<'de> serde::Deserialize<'de> for {rust_type} {{
{indent}    #[allow(deprecated)]
{indent}    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
{indent}    where
{indent}        D: serde::Deserializer<'de>,
{indent}    {{"#,
        indent = Indent(indent),
        rust_type = rust_type
    )
}

fn write_deserialize_end<W: Write>(indent: usize, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        r#"{indent}    }}
{indent}}}"#,
        indent = Indent(indent),
    )
}

fn write_wkt_message_serde<W: Write>(indent: usize, message: &Message, rust_type: &str, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        r#"{indent}#[::pbjson_any::typetag_serde(name = "type.googleapis.com/{full_name}")]
{indent}impl ::pbjson_any::prost_wkt::MessageSerde for {rust_type} {{
{indent}    fn package_name(&self) -> &'static str {{
{indent}        "{package_name}"
{indent}    }}
{indent}    fn message_name(&self) -> &'static str {{
{indent}        "{message_name}"
{indent}    }}
{indent}    fn type_url(&self) -> &'static str {{
{indent}        "type.googleapis.com/{full_name}"
{indent}    }}
{indent}    fn new_instance(&self, data: Vec<u8>) -> Result<Box<dyn ::pbjson_any::prost_wkt::MessageSerde>, ::prost::DecodeError> {{
{indent}        let mut target = Self::default();
{indent}        ::prost::Message::merge(&mut target, data.as_slice())?;
{indent}        let erased: Box<dyn ::pbjson_any::prost_wkt::MessageSerde> = Box::new(target);
{indent}        Ok(erased)
{indent}    }}
{indent}    fn encoded(&self) -> Vec<u8> {{
{indent}        let mut buf = Vec::new();
{indent}        buf.reserve(::prost::Message::encoded_len(self));
{indent}        ::prost::Message::encode(self, &mut buf).expect("Failed to encode message");
{indent}        buf
{indent}    }}
{indent}    fn try_encoded(&self) -> Result<Vec<u8>, ::prost::EncodeError> {{
{indent}        let mut buf = Vec::new();
{indent}        buf.reserve(::prost::Message::encoded_len(self));
{indent}        ::prost::Message::encode(self, &mut buf)?;
{indent}        Ok(buf)
{indent}    }}
{indent}}}"#,
        indent = Indent(indent),
        package_name = message.path.package(),
        message_name = message.path.path().last().unwrap(),
        full_name = message.path,
        rust_type = rust_type
    )
}
