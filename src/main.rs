use decoder::Decoder;
use decoder::ListDecoder;
use decoder_marco::DecoderMacro;

#[derive(DecoderMacro)]
struct BaseDecoder {
    encode_str: &'static str
}

impl ListDecoder for BaseDecoder {}

fn main() {
    let req_url = "";
    
    // for conf in decoder.explode_configs() {
    //     println!("name: {}, config: {}", conf.nameinfo, conf.decode_conf_str);
    // }
}
