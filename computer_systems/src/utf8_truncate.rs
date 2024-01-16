use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};

pub fn utf8_truncate(file: &File) {
  let mut reader = BufReader::new(file);

  let mut buffer: Vec<u8> = Vec::new();

  while reader.read_until(b'\n', &mut buffer).unwrap() != 0 {
    if buffer.len() <= 1 {
      buffer.clear();
      continue;
    }

    let current_byte = &buffer[1..];
    let mut length = buffer[0] as usize;

    if length >= current_byte.len() {
      length = current_byte.len();
    } else {
      while length > 0 && (current_byte[length - 1] & 0xc0) == 0x80 {
        length -= 1;
      }
    }

    io::stdout().write_all(&current_byte[..length]).unwrap();
    io::stdout().write_all(b"\n").unwrap();

    buffer.clear();
  }
}

