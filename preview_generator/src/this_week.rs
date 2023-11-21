use image::RgbaImage;
use rusttype::Font;

use crate::models::Article;
use crate::PreviewGenerator;

pub struct ThisWeekGenerator;

impl PreviewGenerator for ThisWeekGenerator {
    fn gen(
        &self,
        _img: &mut RgbaImage,
        _file_name: String,
        _font: &Font,
        _bold: &Font,
        _article: Article,
        _output: &str,
    ) {
    }
}
