use std::path::PathBuf;

use image::{Pixel, Rgba, RgbaImage};
use rusttype::{Font, Scale};

use crate::components::circle_tag;
use crate::models::Article;
use crate::utils::{append_image, chunked_string};
use crate::{PreviewGenerator, WIDTH};

pub struct ThisWeekGenerator {
    rustlanges: RgbaImage,
    banner: RgbaImage,
}

impl Default for ThisWeekGenerator {
    fn default() -> Self {
        let banner = image::open("assets/RustLangES.png").unwrap().to_rgba8();

        Self {
            rustlanges: image::imageops::resize(
                &banner,
                85,
                85,
                image::imageops::FilterType::Nearest,
            ),
            banner: image::open("assets/banner.png").unwrap().to_rgba8(),
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
        let top = 34;
        let (padding_x, padding_y) = (30i32, 36i32);

        let title_size = 36.;
        let description_size = 60.;

        let _max_title_chars = 30;
        let max_word_wrap = 6;

        let dark_color = Rgba::from_slice(&[253, 186, 116, 255]);
        let tag_color = Rgba::from_slice(&[0xF5, 0xC9, 0x98, 255]);
        let text_color = Rgba::from_slice(&[0x69, 0x37, 0x00, 255]);

        // Paint Background
        image::imageops::vertical_gradient(img, dark_color, dark_color);

        // Banner Image
        append_image(img, &self.banner, 0, 0, 225);

        // Comunity Image
        append_image(
            img,
            &self.rustlanges,
            padding_x as u32,
            top as u32 + padding_y as u32,
            225,
        );

        // Title
        imageproc::drawing::draw_text_mut(
            img,
            text_color.clone(),
            padding_x * 2 + 80,
            top + padding_y + 10,
            Scale::uniform(title_size),
            bold,
            &format!(
                "Semana En Rust #{}",
                article.title.split_once("#").map(|t| t.1).unwrap()
            ),
        );

        // Date
        imageproc::drawing::draw_text_mut(
            img,
            text_color.clone(),
            padding_x * 2 + 80,
            top + padding_y + title_size as i32 + 10,
            Scale::uniform(26.),
            font,
            &article.date.unwrap_or("".to_string()).replace("-", " / ")
        );

        if let Some(tags) = article.tags.as_ref() {
            let mut x = WIDTH as i32 / 2 + 54;
            let y = top + padding_y + 22;

            for tag in tags.iter() {
                let (w, _) = circle_tag(
                    img,
                    bold,
                    24.,
                    tag_color.clone(),
                    text_color.clone(),
                    (x, y),
                    (8, 8),
                    tag.to_string(),
                );
                x += 48 + w;
            }
        }

        // Description
        for (i, s) in chunked_string(article.description, max_word_wrap, 5)
            .iter()
            .enumerate()
        {
            imageproc::drawing::draw_text_mut(
                img,
                text_color.clone(),
                padding_x,
                top + padding_y + 168 + (description_size as i32 * i as i32 + 1),
                Scale::uniform(description_size),
                bold,
                &s,
            );
        }

        // Save
        let mut output = PathBuf::from(&output);
        output.push(format!("{file_name}.png"));

        println!("{output:?}");
        img.save_with_format(output, image::ImageFormat::Png)
            .unwrap();
    }
}
