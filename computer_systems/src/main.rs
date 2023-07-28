use::computer_systems::protobuf_varint;
use::computer_systems::hex_to_rgb;
use::computer_systems::beep_boop;
use::computer_systems::image_blackout;
use::std::fs::File;

fn main() {
    let file = File::open("varint/150.uint64").expect("file not found");

    let encoded_number = protobuf_varint::encode(file);

    let decoded_number = protobuf_varint::decode(encoded_number);

    println!("decoded number: {:?}", decoded_number);

    let file = File::open("color-convert/simple.css").expect("file not found");

    let result = hex_to_rgb::hex_to_rgb(file);

    println!("result: {:?}", result);

    let file = File::open("image-blackout/teapot.bmp").expect("file not found");

    image_blackout::paint_bitmap(&file);

    beep_boop::beep_boop();
}
