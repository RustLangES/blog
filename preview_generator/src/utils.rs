use image::{ImageBuffer, Pixel, Rgba, RgbaImage};

pub fn chunked_string(s: String, words: usize, max_lines: usize) -> Vec<String> {
    s.split_whitespace()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .chunks(words)
        .take(max_lines)
        .enumerate()
        .map(|(i, s)| {
            let s = s.join(" ");
            if i == max_lines - 1 {
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

#[allow(unused)]
pub fn vertical_gradient(
    img: &mut RgbaImage,
    (x, y): (i64, i64),
    (width, height): (u32, u32),
    start: &Rgba<u8>,
    stop: &Rgba<u8>,
)
{
    let mut gradient_img = ImageBuffer::new(width, height);
    image::imageops::vertical_gradient(&mut gradient_img, start, stop);

    image::imageops::overlay(img, &mut gradient_img, x, y);
}
