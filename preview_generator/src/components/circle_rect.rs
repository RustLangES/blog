use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_ellipse_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;

pub fn circle_rect(img: &mut RgbaImage, color: Rgba<u8>, (x, y): (i32, i32), (w, h): (i32, i32)) {
    let half_rad = h / 2;
    // let mut img = img
    //     .view(pos.0 as u32, pos.1 as u32, size.0 as u32, size.1 as u32)
    //     .to_image();
    // Left
    draw_filled_ellipse_mut(img, (x, y + half_rad), half_rad, half_rad, color);
    // Right
    draw_filled_ellipse_mut(
        img,
        (x + w, y + half_rad),
        half_rad,
        half_rad,
        color,
    );

    // main box
    draw_filled_rect_mut(
        img,
        Rect::at(x, y).of_size(w as u32, h as u32 + 2),
        color,
    );
}
