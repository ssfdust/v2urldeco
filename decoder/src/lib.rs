extern crate base64;
use base64::decode;
use std::str;

pub trait Decoder {
    fn get_encode_bytes(&self) -> &[u8];
    fn decode(&self) -> String {
        let encode_bytes = self.get_encode_bytes();
        let decoded = decode(&encode_bytes).unwrap();
        str::from_utf8(&decoded).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
