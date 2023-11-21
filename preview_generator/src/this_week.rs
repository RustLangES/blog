use std::path::PathBuf;

use image::{Pixel, Rgba, RgbaImage};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

use crate::models::Article;
use crate::utils::{append_image, chunked_string};
use crate::{PreviewGenerator, HEIGHT, WIDTH};

pub struct ThisWeekGenerator {
    rustlanges: RgbaImage,
    banner: RgbaImage,
}

impl Default for ThisWeekGenerator {
    fn default() -> Self {
        Self {
            rustlanges: image::open("assets/RustLangES.png").unwrap().to_rgba8(),
            banner: image::open("assets/this_week.jpg").unwrap().to_rgba8(),
        }
    }
}

impl PreviewGenerator for ThisWeekGenerator {
    fn gen(
        &self,
        img: &mut RgbaImage,
        file_name: String,
        font: &Font,
        bold: &Font,
        article: Article,
        output: &str,
    ) {
        let (padding_x, padding_y) = (64, 32);

        let title_size = 64.;
        let description_size = 48.;

        let max_title_chars = 30;
        let max_word_wrap = 6;

        let dark_color = Rgba::from_slice(&[253, 186, 116, 255]);
        let text_color = Rgba::from_slice(&[0, 0, 0, 255]);
        
        // Paint Banner
        append_image(img, &self.banner, 0, 0, 255);

        // Paint Bottom
        imageproc::drawing::draw_filled_rect_mut(
            img,
            Rect::at(0, HEIGHT as i32 / 2).of_size(WIDTH, HEIGHT / 2),
            dark_color.clone(),
        );

        let top = HEIGHT as i32 / 2;

        // Title
        imageproc::drawing::draw_text_mut(
            img,
            text_color.clone(),
            padding_x,
            top + padding_y,
            Scale::uniform(title_size),
            bold,
            &article
                .title
                .get(..max_title_chars)
                .or_else(|| Some(&article.title))
                .map(|s| {
                    if s.len() >= max_title_chars {
                        format!("{s}...")
                    } else {
                        s.to_string()
                    }
                })
                .unwrap(),
        );

        // Description
        for (i, s) in chunked_string(article.description, max_word_wrap, 3)
            .iter()
            .enumerate()
        {
            imageproc::drawing::draw_text_mut(
                img,
                text_color.clone(),
                padding_x,
                top + padding_y + title_size as i32 + 28 + (description_size as i32 * i as i32 + 1),
                Scale::uniform(description_size),
                font,
                &s,
            );
        }

        // Comunity Image
        let x_min = WIDTH - padding_x as u32 - self.rustlanges.width();
        let y_min = HEIGHT - (HEIGHT / 4) - self.rustlanges.height() / 2;
        append_image(img, &self.rustlanges, x_min, y_min, 115);

        // Save
        let mut output = PathBuf::from(&output);
        output.push(format!("{file_name}.png"));

        println!("{output:?}");
        img.save_with_format(output, image::ImageFormat::Png)
            .unwrap();
    }
}
