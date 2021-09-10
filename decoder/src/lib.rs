extern crate base64;
extern crate regex;
use base64::decode;
use regex::Regex;
use std::str;

#[derive(Debug)]
pub struct DecodedCfg {
    pub nameinfo: String,
    pub decode_conf_str: String,
}

pub trait Decoder {
    fn get_encode_bytes(&self) -> &[u8];
    fn decode(&self) -> String {
        decode_to_string(self.get_encode_bytes())
    }
}

fn decode_to_string(encoded: &[u8]) -> String {
    let decoded = decode(&encoded).unwrap();
    str::from_utf8(&decoded).unwrap().to_string()
}

fn padding(encoded_conf_str: String) -> String {
    let mut encoded_conf_str = encoded_conf_str.clone();
    let padding_num = 4 - encoded_conf_str.len() % 4;
    if padding_num != 4 {
        for _ in 0..padding_num {
            encoded_conf_str.push('=');
        }
    }
    encoded_conf_str
}

pub trait ListDecoder: Decoder {
    fn explode_configs(&self) -> Vec<DecodedCfg> {
        let prefix_strip_regex: Regex = Regex::new(r"(vmess|ss)://").unwrap();
        let mut configs: Vec<DecodedCfg> = Vec::new();
        for config_str in self.decode().split('\n') {
            let mut nameinfo = String::new();
            let config_str = prefix_strip_regex.replace_all(config_str, "");
            let decoded_str = decode_to_string(padding(config_str.to_string()).as_bytes());
            if decoded_str.contains("#") {
                let (_config_str, _nameinfo) = decoded_str.split_once('#').unwrap();
                nameinfo.push_str(_nameinfo);
            }
            configs.push(DecodedCfg {
                nameinfo: nameinfo.to_owned(),
                decode_conf_str: decoded_str.to_owned()
            });
        }
        configs
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
