use std::{path::Path, io::Cursor};
use std::fs::read;

use image::{ImageBuffer, Rgba, load, ImageFormat};

pub struct Sprite {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    dim: (u32, u32)
}

impl Sprite {
    /// Loads an image from a path
    pub fn fromPath<P: AsRef<Path>>(path: P) -> Sprite {
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = load(Cursor::new(&read(path).unwrap()[..]), ImageFormat::Png).unwrap().to_rgba8();
        Self {
            dim: image.dimensions(),
            image
        }
    }

    /// Loads an image from bytes
    pub fn fromBytes(bytes: &[u8]) -> Sprite {
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = load(Cursor::new(&bytes[..]), ImageFormat::Png).unwrap().into_rgba8();

        Sprite {
            dim: image.dimensions(),
            image,
        }
    }

    pub fn fromVecBytes(bytes: &Vec<u8>) -> Sprite {
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = load(Cursor::new(&bytes[..]), ImageFormat::Png).unwrap().into_rgba8();

        Sprite {
            dim: image.dimensions(),
            image
        }
    }
}