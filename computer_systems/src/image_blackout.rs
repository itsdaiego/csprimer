use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};

pub fn paint_bitmap(mut file: &File) {
    let mut bmp_header = [0; 14];
    let mut dib_header = [0; 40];

    file.read_exact(&mut bmp_header).unwrap();
    file.read_exact(&mut dib_header).unwrap();

    let pixel_data_offset = u32::from_le_bytes([bmp_header[10], bmp_header[11], bmp_header[12], bmp_header[13]]);
    let width = u32::from_le_bytes([dib_header[4], dib_header[5], dib_header[6], dib_header[7]]);
    let height = u32::from_le_bytes([dib_header[8], dib_header[9], dib_header[10], dib_header[11]]);

    let mut new_file = File::create("image-blackout/pitch-black.bmp").unwrap();
    new_file.write_all(&bmp_header).unwrap();
    new_file.write_all(&dib_header).unwrap();

    let row_size = ((width * 3 + 3) / 4) * 4;

    for _ in 0..height {
        let row = vec![0; row_size as usize];
        new_file.write_all(&row).unwrap();
    }

    file.seek(SeekFrom::Start(pixel_data_offset as u64)).unwrap();
    let mut rest_of_file = Vec::new();

    file.read_to_end(&mut rest_of_file).unwrap();
    new_file.write_all(&rest_of_file).unwrap();
}
