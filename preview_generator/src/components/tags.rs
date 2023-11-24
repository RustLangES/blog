use image::{RgbaImage, Rgba};
use imageproc::drawing::text_size;
use rusttype::{Font, Scale};

use super::circle_rect;
use super::rounded_rect::rounded_rect;

pub fn rounded_tag(
    img: &mut RgbaImage,
    font: &Font,
    font_size: f32,
    bg_color: Rgba<u8>,
    text_color: Rgba<u8>,
    radius: i32,
    pos: (i32, i32),
    padding: (i32, i32),
    content: String,
) -> (i32, i32) {
    let (text_width, text_height) = text_size(Scale::uniform(font_size), font, &content);
    let size = ((padding.0 * 2) + text_width, (padding.1 * 2) + text_height);

    rounded_rect(img, bg_color, radius, pos, size);

    imageproc::drawing::draw_text_mut(
        img,
        text_color,
        padding.0 + pos.0,
        padding.1 + pos.1,
        Scale::uniform(font_size),
        font,
        &content,
    );

    size
}

pub fn circle_tag(
    img: &mut RgbaImage,
    font: &Font,
    font_size: f32,
    bg_color: Rgba<u8>,
    text_color: Rgba<u8>,
    pos: (i32, i32),
    padding: (i32, i32),
    content: String,
) -> (i32, i32) {
    let (text_width, text_height) = text_size(Scale::uniform(font_size), font, &content);
    let size = ((padding.0 * 2) + text_width, (padding.1 * 2) + text_height);

    circle_rect(img, bg_color, pos, size);

    imageproc::drawing::draw_text_mut(
        img,
        text_color,
        padding.0 + pos.0,
        padding.1 + pos.1,
        Scale::uniform(font_size),
        font,
        &content,
    );

    size
}

