use image::RgbImage;

use crate::PreviewGenerator;
use crate::models::Article;

pub struct BlogGenerator;

impl PreviewGenerator for BlogGenerator {
    fn gen(&self, img: &mut RgbImage, article: Article, output: &str) {
    }
}
