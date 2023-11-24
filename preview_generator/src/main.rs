use std::env::args;
use std::fs;

use gray_matter::engine::YAML;
use gray_matter::Matter;
use image::RgbaImage;
use models::Article;
use rusttype::Font;

mod blog;
mod components;
mod models;
mod this_week;
mod utils;

pub const WIDTH: u32 = 1200;
pub const HEIGHT: u32 = 630;

const REGULAR_FONT_BYTES: &[u8] = include_bytes!("../assets/WorkSans-Regular.ttf");
const BOLD_FONT_BYTES: &[u8] = include_bytes!("../assets/WorkSans-Bold.ttf");

pub trait PreviewGenerator {
    fn gen(
        &self,
        img: &mut RgbaImage,
        file_name: String,
        font: &Font,
        bold: &Font,
        article: Article,
        output: &str,
    );
}

fn main() {
    let mut argv = args().skip(1);
    let Some(file_type) = argv.next() else {
        panic!("No hay el typo de arhivo en los argumentos");
    };
    let Some(folder) = argv.next() else {
        panic!("No hay la carpeta en los argumentos");
    };
    let Some(output) = argv.next() else {
        panic!("No hay la carpeta de salida en los argumentos");
    };

    match file_type.as_str() {
        "blog" => generate(folder, output, blog::BlogGenerator::default()),
        "this_week" => generate(folder, output, this_week::ThisWeekGenerator::default()),
        x => panic!("El tipo de archivo '{x}' no es admitido"),
    };
}

pub fn generate<G: PreviewGenerator>(folder: String, output: String, generator: G) {
    let folder_content = fs::read_dir(folder).unwrap();
    let font = Font::try_from_bytes(REGULAR_FONT_BYTES).unwrap();
    let bold = Font::try_from_bytes(BOLD_FONT_BYTES).unwrap();

    for post_file in folder_content {
        let post_file = post_file.unwrap();
        let file_name = post_file.file_name().to_str().unwrap().to_string();
        let file = post_file.path();
        let content = fs::read_to_string(file).unwrap();
        let matter = Matter::<YAML>::new();
        let Some(article) = matter.parse_with_struct::<Article>(&content) else {
            continue;
        };

        let mut img = RgbaImage::new(WIDTH, HEIGHT);
        generator.gen(
            &mut img,
            file_name.replace(".md", ""),
            &font,
            &bold,
            article.data,
            &output,
        );
    }
}
