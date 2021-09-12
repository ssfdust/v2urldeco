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

fn second_parse_config(config_str: &str) -> String {
    let prefix_strip_regex: Regex = Regex::new(r"(vmess|ss)://").unwrap();
    let config_str = prefix_strip_regex.replace_all(config_str, "");
    decode_to_string(padding(config_str.to_string()).as_bytes())
}

pub trait ListDecoder: Decoder {
    fn explode_configs(&self) -> Vec<DecodedCfg> {
        let mut configs: Vec<DecodedCfg> = Vec::new();
        let mut nameinfo = String::new();
        for config_str in self.decode().split('\n') {
            let mut decoded_str = second_parse_config(config_str);
            if decoded_str.contains("#") {
                match decoded_str.split_once('#') {
                    None => (),
                    Some((x, y)) => {
                        nameinfo.push_str(x);
                        decoded_str = y.to_owned();
                    }
                }
            }
            configs.push(DecodedCfg {
                nameinfo: nameinfo.clone(),
                decode_conf_str: decoded_str.clone()
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
