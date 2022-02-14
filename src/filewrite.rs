use crate::{
    constants::{HEIGHT, WIDTH},
    types::Image,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

// Write the image to a TGA file with the given name.
// Format specification: http://www.gamers.org/dEngine/quake3/TGA.txt
pub fn write_tga(filename: &str, image: &Image) {
    let file = File::create(filename).unwrap();
    // use a buffered writer
    let mut writer = BufWriter::new(file);

    // Write the TGA header.
    #[rustfmt::skip]
    let header: [u8; 18] = [
        0, // no image ID
        0, // no colour map
        2, // uncompressed 24-bit image
        0, 0, 0, 0, 0, // empty colour map specification
        0, 0, // X origin
        0, 0, // Y origin
        WIDTH as u8, (WIDTH >> 8) as u8, // width
        HEIGHT as u8, (HEIGHT >> 8) as u8, // height
        24, // bits per pixel
        0, // image descriptor
    ];

    writer.write_all(&header).unwrap();

    for row in image.iter() {
        for &loc in row.iter() {
            let pixel: [u8; 3] = [
                (loc & 0xFF) as u8,
                (loc >> 8 & 0xFF) as u8,
                (loc >> 16 & 0xFF) as u8,
            ];
            writer.write_all(&pixel).unwrap();
        }
    }

    println!("Wrote {}", filename);
}
