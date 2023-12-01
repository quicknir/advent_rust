use std::{path::{Path, PathBuf}, fs::File};
use std::io::{BufReader, BufRead};
use reqwest::{blocking::Client, header::COOKIE};

fn read_lines<P: AsRef<Path>>(filename: P) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines().map(|x| x.unwrap())
}

pub fn read_aoc_lines_impl(root_dir: &str, src_filename: &str) -> impl Iterator<Item = String> {
    let mut p = PathBuf::from(root_dir);
    p.push("input");
    p.push(Path::new(src_filename).file_stem().unwrap());
    p.set_extension("txt");
    if !p.exists() {
        download_file(root_dir, &p);
    }
    read_lines(p)
}

fn download_file(root_dir: &str, target_filename: &Path) {
    let year: i16 = Path::new(root_dir).file_stem().unwrap().to_str().unwrap().strip_prefix("advent_").unwrap().parse().unwrap();
    let cookie_path = target_filename.parent().unwrap().join("cookie.txt");
    let cookie = read_lines(&cookie_path).next().unwrap();
    let day: i8 = target_filename.file_stem().unwrap().to_str().unwrap().get(3..).unwrap().parse().unwrap();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = Client::new();
    let result = client.get(url).header(COOKIE,cookie).send().unwrap().text().unwrap();
    std::fs::write(target_filename, &result).unwrap();
}

#[macro_export]
macro_rules! read_aoc_lines {
    () => {
        utils::read_aoc_lines_impl(env!("CARGO_MANIFEST_DIR"), file!())
    };
}