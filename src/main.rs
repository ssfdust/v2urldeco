use decoder::Decoder;
use decoder_marco::DecoderMacro;

#[derive(DecoderMacro)]
struct BaseDecoder {
    encode_str: &'static str
}


fn main() {
    let decoder = BaseDecoder{ encode_str: "YWFh" };
    println!("Hello, world {}!", decoder.decode());
}
