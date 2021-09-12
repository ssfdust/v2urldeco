use std::ffi::OsString;
use std::env;
use decoder::Decoder;
use decoder::ListDecoder;
use decoder_marco::DecoderMacro;
use fetcher::fetch_resp_text;

#[derive(DecoderMacro)]
struct BaseDecoder {
    encode_str: String
}

impl ListDecoder for BaseDecoder {}

fn main() {
    let url = match env::var_os("REMOTE_URL"){
        None => OsString::from("http//127.0.0.1").to_owned(),
        Some(url) => url
    };
    let text = fetch_resp_text(url.to_str().unwrap().to_owned().as_str());
    let decoder = BaseDecoder{encode_str: text};
    
    for conf in decoder.explode_configs() {
        println!("name: {}, config: {}", conf.nameinfo, conf.decode_conf_str);
    }
}
