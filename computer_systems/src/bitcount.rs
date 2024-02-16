pub fn count_on_bits (mut number: u32) -> u32 {
  let mut count = 0;

  while number != 0 {
    count +=  number & 1;
    number = number >> 1;
  }

  count
}

// pub fn count_on_bits (number: u32) -> u32 {
//   let bytes = number.to_be_bytes();

//   let mut count = 0;

//   for byte in bytes.iter() {
//     count = byte.count_ones();
//   }

//   count
// }

