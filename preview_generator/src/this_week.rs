use std::fs::File;
use std::io::BufReader;
use rss::Channel;

pub fn generate(rss_file: String) {
    let file = File::open(rss_file).unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();
}
