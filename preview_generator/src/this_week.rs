use image::RgbImage;

use crate::models::Article;
use crate::PreviewGenerator;

pub struct ThisWeekGenerator;

impl PreviewGenerator for ThisWeekGenerator {
    fn gen(&self, img: &mut RgbImage, article: Article, output: &str) {}
}
