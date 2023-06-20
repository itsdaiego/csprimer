use::std::fs::File;
use::std::io::Read;


pub fn encode(file: File) -> Vec<u8> {
    let mut reader = std::io::BufReader::new(file);

    let mut buffer = [0; 8];

    reader.read_exact(&mut buffer).expect("buffer overflow");

    let number_read = u64::from_be_bytes(buffer);

    let mut bytes = Vec::new();

    let mut number = number_read;

    while number > 0 {
        let byte = (number & 0x7f) as u8;
        number >>= 7;

        if number > 0 {
            bytes.push(byte | 0x80);
        } else {
            bytes.push(byte);
        }
    }

    println!("bytes: {:x?}", bytes);

    return bytes
}


pub fn decode(encoded_varint: Vec<u8>) -> u64 {
    let mut number = 0;

    println!("encoded varint: {:x?}", encoded_varint);

    for byte in encoded_varint.iter().rev() {
        number <<= 7;

        number |=  (byte & 0x7f) as u64;

        println!("decoded number: {:?}", number);
    }

    println!("decoded number: {:?}", number);

    return number
}
