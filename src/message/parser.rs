use bytes::Bytes;
use super::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

const PARSERS: Lazy<HashMap<(MessageSource, i16), Box<dyn MessageParserTrait>>> = Lazy::new(|| {
    super::super::entity::get_all_parsers().into_iter().map(|e| ((e.source(), e.command().cid()), e)).collect()
});

pub fn parse_data(src: MessageSource, cid: i16, data: &Bytes) -> Result<Option<String>, SerdeError> {
    Ok(match PARSERS.get(&(src, cid)) {
        Some(parser) => {
            Some(parser.parse(&data)?)
        }
        None => {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use bytes::Buf;
    use crate::entity::NotifyBeatCaptainRsp;
    use super::*;

    #[test]
    fn test() {
        let d = "b4a6415502000000";
        let mut data = Bytes::from(hex::decode(d).unwrap());
        let r = parse_data(MessageSource::Client, 1269, &data);
        println!("{:?}", r);
    }
}