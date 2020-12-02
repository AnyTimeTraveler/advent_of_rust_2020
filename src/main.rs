use std::fs::File;
use std::io::Read;

mod day01;

fn main() {
    let day = 1;

    let mut input = File::open(format!("input/{:02}.txt", day)).unwrap();

    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();
    println!("Day {:02}: {}", day, day01::second(content));
}
