extern crate image;
extern crate glium_graphics;

use glium_graphics::{Glium2d, GliumGraphics, GliumWindow, GlyphCache, Texture, TextureSettings};
use image::{RgbImage, ConvertBuffer};


fn run() {
    let mut imgbuf = RgbImage::new(100, 100);
    let texture = Texture::from_image(&mut glium_window,
                                      &imgbuf.convert(),
                                      &TextureSettings::new());
}
