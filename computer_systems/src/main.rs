use::computer_systems::protobuf_varint;
use::std::fs::File;

fn main() {
    let file = File::open("varint/150.uint64").expect("file not found");

    let encoded_number = protobuf_varint::encode(file);

    protobuf_varint::decode(encoded_number);
}
