use image::{Pixel, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_ellipse_mut, draw_filled_rect_mut, text_size};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

pub fn chunked_string(s: String, size: usize, max: usize) -> Vec<String> {
    s.split_whitespace()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .chunks(size)
        .take(max)
        .enumerate()
        .map(|(i, s)| {
            let s = s.join(" ");
            if i == max - 1 {
                format!("{s}...")
            } else {
                s
            }
        })
        .collect::<Vec<String>>()
}

pub fn append_image(img: &mut RgbaImage, other: &RgbaImage, x_min: u32, y_min: u32, new_alpha: u8) {
    let mut logo_x = 0;
    for x in x_min..x_min + other.width() {
        let mut logo_y = 0;
        for y in y_min..y_min + other.height() {
            let pixel = other.get_pixel(logo_x, logo_y);
            logo_y += 1;
            if pixel.0[3] == 0 {
                continue;
            }
            let old_pixel = img.get_pixel_mut(x, y);
            old_pixel.blend(&pixel.map_with_alpha(|c| c, |_| new_alpha));
        }
        logo_x += 1;
    }
}

pub fn make_tag(
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

    rect_corner(img, bg_color, radius, pos, size);

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

pub fn rect_corner(
    img: &mut RgbaImage,
    color: Rgba<u8>,
    radius: i32,
    (x, y): (i32, i32),
    (w, h): (i32, i32),
) {
    let half_rad = radius / 2;
    // let mut img = img
    //     .view(pos.0 as u32, pos.1 as u32, size.0 as u32, size.1 as u32)
    //     .to_image();
    // Top Left
    draw_filled_ellipse_mut(img, (x + radius, y + half_rad), radius, radius, color);
    // Top Right
    draw_filled_ellipse_mut(img, (x + w - half_rad, y + half_rad), radius, radius, color);
    // Bottom Left
    draw_filled_ellipse_mut(img, (x + radius, y + h - half_rad), radius, radius, color);
    // Bottom Right
    draw_filled_ellipse_mut(
        img,
        (x + w - half_rad, y + h - half_rad),
        radius,
        radius,
        color,
    );

    // main box
    draw_filled_rect_mut(
        img,
        Rect::at(x + radius, y + radius)
            .of_size(w as u32 - radius as u32, h as u32 - radius as u32),
        color,
    );

    // top box
    draw_filled_rect_mut(
        img,
        Rect::at(x + radius, y - half_rad).of_size(w as u32 - radius as u32, radius as u32 * 2),
        color,
    );
    // left box
    draw_filled_rect_mut(
        img,
        Rect::at(x, y + half_rad).of_size(radius as u32, h as u32 - half_rad as u32 * 2),
        color,
    );
    // right box
    draw_filled_rect_mut(
        img,
        Rect::at(x + w - half_rad + 2, y + half_rad)
            .of_size(radius as u32, h as u32 - half_rad as u32 * 2),
        color,
    );
    // bottom box
    draw_filled_rect_mut(
        img,
        Rect::at(x + radius, y + h - radius - 3).of_size(w as u32 - radius as u32, radius as u32 * 2),
        color,
    );
}
