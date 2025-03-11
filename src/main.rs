use std::{fs::File, io::BufReader, io::BufRead};

fn main() {
    let file = File::open("setup.csv").unwrap();

    let lines = BufReader::new(file).lines();
    let lines: Vec<String>= lines.map(|l| l.expect("failed reading line")).collect();

}

