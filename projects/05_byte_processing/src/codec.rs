use std::marker::PhantomData;

use serde::{Serialize, de::DeserializeOwned};
use tokio_util::{
    bytes::{BufMut, BytesMut},
    codec::{Decoder, Encoder},
};

pub struct JsonLinesCodec<S>(PhantomData<S>);

impl<S> JsonLinesCodec<S> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<S: Serialize> Encoder<S> for JsonLinesCodec<S> {
    type Error = std::io::Error;

    fn encode(&mut self, item: S, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // json encode the item into the buffer
        serde_json::to_writer(dst.writer(), &item)?;
        // add new-line separator
        dst.put_u8(b'\n');
        Ok(())
    }
}

impl<S: DeserializeOwned> Decoder for JsonLinesCodec<S> {
    type Item = S;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let Some(new_line) = src.iter().position(|&b| b == b'\n') else {
            // not enough data ready yet
            return Ok(None);
        };

        // remove full line from the buffer
        let line_end = new_line + 1;
        let json_line = src.split_to(line_end).freeze();

        // parse the json line.
        let item = serde_json::from_slice(&json_line)?;

        Ok(Some(item))
    }
}
