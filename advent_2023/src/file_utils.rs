use reqwest::{blocking::Client, header::COOKIE};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub fn read_aoc_impl(root_dir: &str, src_filename: &str) -> String {
    let mut p = PathBuf::from(root_dir);
    p.push("input");
    p.push(Path::new(src_filename).file_stem().unwrap());
    p.set_extension("txt");
    if !p.exists() {
        download_file(root_dir, &p);
    }
    read_to_string(p).unwrap()
}

fn download_file(root_dir: &str, target_filename: &Path) {
    let year: i16 = Path::new(root_dir)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix("advent_")
        .unwrap()
        .parse()
        .unwrap();
    let cookie_path = target_filename.parent().unwrap().join("cookie.txt");
    let cookie = read_to_string(&cookie_path).unwrap();
    let day: i8 = target_filename
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .get(3..)
        .unwrap()
        .parse()
        .unwrap();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = Client::new();
    let result = client
        .get(url)
        .header(COOKIE, cookie)
        .send()
        .unwrap()
        .text()
        .unwrap();
    std::fs::write(target_filename, &result).unwrap();
}

#[macro_export]
macro_rules! read_aoc {
    () => {
        utils::read_aoc_impl(env!("CARGO_MANIFEST_DIR"), file!())
    };
}