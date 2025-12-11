use crate::utils::{ByteTypes, WriterStreams};

pub fn toml_decoder(content: Vec<u8>) -> WriterStreams<impl Iterator<Item = ByteTypes>> {
    WriterStreams::LineByLine { iter: std::iter::once(ByteTypes::Raw(content)) }
}
