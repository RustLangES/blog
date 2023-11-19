use std::env::args;

mod blog;
mod this_week;

fn main() {
    let mut argv = args();
    let Some(file_type) = argv.next() else {
        panic!("No hay el typo de arhivo en los argumentos");
    };
    let Some(rss_file) = argv.next() else {
        panic!("No hay el archivo rss en los argumentos");
    };

    match file_type.as_str() {
        "blog" => blog::generate(rss_file),
        "this_week" => this_week::generate(rss_file),
        x => panic!("El tipo de archivo '{x}' no es admitido"),
    }
}
