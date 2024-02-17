// use::computer_systems::protobuf_varint;
// use::computer_systems::hex_to_rgb;
// use::computer_systems::beep_boop;
// use::computer_systems::image_blackout;
// use::computer_systems::syn_flood;
// use::computer_systems::utf8_truncate;
// use::computer_systems::bitcount;
use std::fs::File;
use std::io::{BufRead, BufReader};

use::computer_systems::fast_pangram;

fn main() {
    // let file = File::open("varint/150.uint64").expect("file not found");

    // let encoded_number = protobuf_varint::encode(file);

    // let decoded_number = protobuf_varint::decode(encoded_number);

    // println!("decoded number: {:?}", decoded_number);

    // let file = File::open("color-convert/simple.css").expect("file not found");

    // let result = hex_to_rgb::hex_to_rgb(file);

    // println!("result: {:?}", result);

    // let file = File::open("image-blackout/teapot.bmp").expect("file not found");

    // image_blackout::paint_bitmap(&file);

    // beep_boop::beep_boop();

    // let file = File::open("syn-flood/synflood.pcap").expect("file not found");

    // syn_flood::detect_flood(&file);

    // let file = File::open("utf8-truncate/cases").expect("file not found");

    // utf8_truncate::utf8_truncate(&file);


    // println!("{:?}", bitcount::count_on_bits(1));
    // println!("{:?}", bitcount::count_on_bits(3));
    // println!("{:?}", bitcount::count_on_bits(23));


    let file = File::open("fast-pangram/pangrams.txt").expect("file not found");

    for line in BufReader::new(file).lines() {
        let sentence = line.unwrap();

        let is_fast_pangram = fast_pangram::run(sentence.as_str());

        println!("Fast pangram result {}", is_fast_pangram);
    }
}
