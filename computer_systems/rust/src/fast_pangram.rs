use std::collections::HashMap;

pub fn run (text: &str) -> bool {
  println!("show text: {:?}", text);

  let text_vec: Vec<char> = text.chars().collect();

  let ascii_lowercase_sum: u32 = 2847;

  let mut sum = 0;

  let mut map = HashMap::new();

  for c in text_vec.iter() {
    let ascii = c.to_ascii_lowercase() as u8;

    if !ascii.is_ascii_alphabetic() {
      continue;
    }

    if map.get(&ascii).is_none() {
      map.insert(ascii, true);

      sum += ascii as u32;
    }
  }

  println!("sum: {:?}", sum);

  sum == ascii_lowercase_sum
}
