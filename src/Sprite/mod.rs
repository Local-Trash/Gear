use std::{path::Path, io::Cursor};
use std::fs::read;

use image::{ImageBuffer, Rgba, load, ImageFormat};

pub struct Sprite {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    dim: (u32, u32)
}

impl Sprite {
    pub fn new<P: AsRef<Path>>(path: P) -> Sprite {
        let image =load(Cursor::new(&read(path).unwrap()[..]), ImageFormat::Png).unwrap().to_rgba8();
        Self {
            dim: image.dimensions(),
            image
        }
    }
}