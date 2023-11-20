use std::env::args;
use std::fs;

use gray_matter::engine::YAML;
use gray_matter::Matter;
use image::RgbImage;
use models::Article;

mod blog;
mod models;
mod this_week;

pub const WIDTH: u32 = 1200;
pub const HEIGHT: u32 = 630;

pub trait PreviewGenerator {
    fn gen(&self, img: &mut RgbImage, article: Article, output: &str);
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

    let generator: &dyn PreviewGenerator = match file_type.as_str() {
        "blog" => &blog::BlogGenerator,
        "this_week" => &this_week::ThisWeekGenerator,
        x => panic!("El tipo de archivo '{x}' no es admitido"),
    };

    generate(folder, output, generator)
}

pub fn generate(folder: String, output: String, generator: &dyn PreviewGenerator) {
    let folder_content = fs::read_dir(folder).unwrap();

    for post_file in folder_content {
        let file = post_file.unwrap().path();
        let content = fs::read_to_string(file).unwrap();
        let matter = Matter::<YAML>::new();
        let Some(article) = matter.parse_with_struct::<Article>(&content) else {
            continue;
        };

        let mut img = RgbImage::new(WIDTH, HEIGHT);
        generator.gen(&mut img, article.data, &output);
    }
}
