use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::{Seek, SeekFrom};

pub fn detect_flood(file: &File) {
  // reading syn_flood pcap file header
  let mut reader = BufReader::new(file);

  reader.seek(SeekFrom::Start(4)).unwrap();

  let mut major_version_buffer = [0; 2];

  reader.read_exact(&mut major_version_buffer).unwrap();

  let major_version = u16::from_le_bytes(major_version_buffer);

  let mut minor_version_buffer = [0; 2];

  reader.read_exact(&mut minor_version_buffer).unwrap();

  let minor_version = u16::from_le_bytes(minor_version_buffer);

  println!("major and minor version: {:?}.{:?}", major_version, minor_version);

  reader.seek(SeekFrom::Start(24)).unwrap();

  let mut index = 0;

  let mut syn_count = 0;
  let mut ack_count = 0;

  loop {
    index += 1;
    println!("count parsed packets: {}", index);

    // reading syn_flod pcap reader per header
    let mut per_packet_header_buffer = [0; 16];

    match reader.read_exact(&mut per_packet_header_buffer) {
      Ok(()) => {
        // Successfully read the header
        let current_packet_timestamp_in_seconds_sector = &per_packet_header_buffer[0..4];
        let current_packet_timestamp_in_seconds = u32::from_le_bytes(current_packet_timestamp_in_seconds_sector.try_into().unwrap());

        if current_packet_timestamp_in_seconds == 0 {
          println!("--------------------");
          println!("FINISHED READING DATA :P");
          println!("--------------------");
          break;
        }

        let position = reader.stream_position().unwrap();
        println!("CURRENT POSITION: {:?}", position); // 32
        println!("current packet timestamp {}", current_packet_timestamp_in_seconds);

        let included_length_sector = &per_packet_header_buffer[8..12];
        let included_length = u32::from_le_bytes(included_length_sector.try_into().unwrap());

        let mut packet_data_header_buffer = vec![0; included_length as usize];

        reader.read_exact(&mut packet_data_header_buffer).unwrap();

        let ip_version = (packet_data_header_buffer[4] & 0xF0) >> 4; // or simply shift >>4

        let ihl = (((packet_data_header_buffer[4] & 0x0F) << 2) + 4) as usize;
        
        let tcp_header = &packet_data_header_buffer[ihl..39];

        let source_port = u16::from_be_bytes(tcp_header[0..2].try_into().unwrap());
        let destination_port = u16::from_be_bytes(tcp_header[2..4].try_into().unwrap());

        let tcp_flags = tcp_header[14]; // 39 (exclusive) - ihl = 24 = 14

        let is_syn_set = (tcp_flags & 0x0002) > 0;
        let is_ack_set = (tcp_flags & 0x0010) > 0;

        if is_syn_set && destination_port == 80 {
          syn_count += 1;
        }

        if is_ack_set && source_port == 80 {
          ack_count += 1;
        }

        println!("source port {:?}", source_port);
        println!("ip header version: {:?}", ip_version);
        println!("internet header length: {:?}", ihl);
        println!("included length: {:?}", included_length);
        println!("source and destination ports: {:?} {:?}", source_port, destination_port);
        println!("number of SYN vs ACK: {:?} {:?} --- {:?}% acked", syn_count, ack_count, ((ack_count as f64 / syn_count as f64) * 100.0));
        println!("--------------------");
      },
      Err(e) => {
        // If an error occurred while reading
        if e.kind() == std::io::ErrorKind::UnexpectedEof {
          println!("--------------------");
          println!("Reached the end of file while reading a packet header.");
          println!("--------------------");
          break;
        } else {
          // Other kinds of errors
          panic!("An error occurred: {}", e);
        }
      }
    }
  }

}
