use std::{path::{Path, PathBuf}, fs::File};
use std::io::{BufReader, BufRead};
use reqwest::{blocking::Client, header::COOKIE};

fn read_lines<P: AsRef<Path>>(filename: P) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines().map(|x| x.unwrap())
}

pub fn read_aoc_lines_impl(year: i16, root_dir: &str, src_filename: &str) -> impl Iterator<Item = String> {
    let mut p = PathBuf::from(root_dir);
    p.push(Path::new(src_filename).file_stem().unwrap());
    p.set_extension("txt");
    if !p.exists() {
        download_file(year, &p);
    }
    read_lines(p)
}

fn download_file(year: i16, target_filename: &Path) {
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
        utils::read_aoc_lines_impl(2023, "/home/nir/Downloads/advent_2023", file!())
    };
}