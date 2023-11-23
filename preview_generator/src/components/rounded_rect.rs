use image::{RgbaImage, Rgba};
use imageproc::drawing::{draw_filled_ellipse_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;

pub fn rounded_rect(
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
        Rect::at(x + radius, y + h - radius - 3)
            .of_size(w as u32 - radius as u32, radius as u32 * 2),
        color,
    );
}
