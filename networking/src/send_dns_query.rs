use std::io::Cursor;
use std::net::UdpSocket;
use byteorder::{BigEndian, ReadBytesExt};

pub fn solve() {
    let socket = UdpSocket::bind("0.0.0.0:3050").unwrap();
    println!("Server listening on port 3050");


    let transaction_id = [0x00, 0x01];
    
    let flags = [0x00, 0x01];
    let questions = [0x00, 0x01];
    let answer_rrs = [0x00, 0x00];
    let authority_rrs = [0x00, 0x00];
    let additional_rrs = [0x00, 0x00];

    let domain = String::from("www.wikipedia.org");

    let mut domain_encoded = Vec::new();

    for label in domain.split('.') {
        let label_length = label.len();
        domain_encoded.push(label_length as u8);
        domain_encoded.extend_from_slice(label.as_bytes());
    }

    domain_encoded.push(0); 

    let query_type = [0x00, 0x01];
    let query_class = [0x00, 0x01];

    let mut buffer = Vec::new();

    buffer.extend_from_slice(&transaction_id);
    buffer.extend_from_slice(&flags);
    buffer.extend_from_slice(&questions);
    buffer.extend_from_slice(&answer_rrs);
    buffer.extend_from_slice(&authority_rrs);
    buffer.extend_from_slice(&additional_rrs);
    buffer.extend_from_slice(&domain_encoded);
    buffer.extend_from_slice(&query_type);
    buffer.extend_from_slice(&query_class);

    socket.send_to(&buffer, "8.8.8.8:53").unwrap();

    let mut response_buffer = [0u8; 4096];

    let (num_of_bytes, src_addr) = socket.recv_from(&mut response_buffer).expect("Didn't receive data");

    println!("{:?}", &response_buffer[..num_of_bytes]);

    println!("received {} bytes from {}", num_of_bytes, src_addr);

    let mut cursor = Cursor::new(response_buffer);

    cursor.set_position(6);

    let answer_count = cursor.read_u16::<BigEndian>().unwrap();
    cursor.set_position(12);

    let mut label_length = cursor.read_u8().unwrap();

    while label_length != 0 {
        cursor.set_position(cursor.position() + label_length as u64);
        label_length = cursor.read_u8().unwrap();
    }

    cursor.set_position(cursor.position() + 4);


    println!("answer_count: {}", answer_count);

    for _ in 0..answer_count {
        cursor.set_position(cursor.position() + 10);

        let data_length = cursor.read_u16::<BigEndian>().unwrap();

        if data_length == 4 {
            let ip_bytes = [
                cursor.read_u8().unwrap(),
                cursor.read_u8().unwrap(),
                cursor.read_u8().unwrap(),
                cursor.read_u8().unwrap(),
            ];

            println!("IP address: {}.{}.{}.{}", ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
        } else {
            cursor.set_position(cursor.position() + data_length as u64);
        }
    }
}
