use std::path::PathBuf;

use image::{Pixel, Rgba, RgbaImage};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

use crate::models::Article;
use crate::utils::{append_image, chunked_string, make_tag};
use crate::{PreviewGenerator, HEIGHT, WIDTH};

pub struct BlogGenerator;

impl PreviewGenerator for BlogGenerator {
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

        let bg_color = Rgba::from_slice(&[254, 215, 170, 255]);
        let dark_color = Rgba::from_slice(&[253, 186, 116, 255]);
        let text_color = Rgba::from_slice(&[0, 0, 0, 255]);
        let tag_bg_color = Rgba::from_slice(&[255, 255, 255, 255]);
        let tag_text_color = Rgba::from_slice(&[249, 115, 22, 255]);

        // TODO: make efficient implementation
        let rustlanges = image::open("assets/RustLangES.png").unwrap();
        let user_img = image::open("assets/user.png").unwrap();
        let tag_img = image::open("assets/tag.png").unwrap();

        // Paint Background
        image::imageops::vertical_gradient(img, bg_color, bg_color);

        // Build Top
        // Title
        imageproc::drawing::draw_text_mut(
            img,
            text_color.clone(),
            padding_x,
            padding_y,
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
                padding_y + title_size as i32 + 28 + (description_size as i32 * i as i32 + 1),
                Scale::uniform(description_size),
                font,
                &s,
            );
        }

        // Build Bottom
        imageproc::drawing::draw_filled_rect_mut(
            img,
            Rect::at(0, HEIGHT as i32 / 2).of_size(WIDTH, HEIGHT / 2),
            dark_color.clone(),
        );
        // User Section
        append_image(
            img,
            &user_img.to_rgba8(),
            padding_x as u32,
            padding_y as u32 + (HEIGHT / 2) + 48,
            255,
        );
        imageproc::drawing::draw_text_mut(
            img,
            text_color.clone(),
            padding_x + 65,
            padding_y + (HEIGHT as i32 / 2) + 48,
            Scale::uniform(description_size),
            font,
            &article.author.unwrap_or("Desconocido".to_string())
        );

        // Tags Section
        if let Some(tags) = article.tags.as_ref() {
            let mut x = padding_x + 65;
            let y = padding_y + (HEIGHT as i32 / 2) + 12 + 48 * 2;

            append_image(img, &tag_img.to_rgba8(), padding_x as u32, y as u32, 255);

            for tag in tags.iter() {
                let (w, _) = make_tag(
                    img,
                    font,
                    24.,
                    tag_bg_color.clone(),
                    tag_text_color.clone(),
                    8,
                    (x, y),
                    (8, 8),
                    tag.to_string(),
                );
                x += 12 + w;
            }
        }

        // Comunity Image
        let x_min = WIDTH - padding_x as u32 - rustlanges.width();
        let y_min = HEIGHT - (HEIGHT / 4) - rustlanges.height() / 2;
        append_image(img, &rustlanges.to_rgba8(), x_min, y_min, 115);

        // Save
        let mut output = PathBuf::from(&output);
        output.push(format!("{file_name}.png"));

        println!("{output:?}");
        img.save_with_format(output, image::ImageFormat::Png)
            .unwrap();
    }
}
