use std::fs::File;
use std::io::Read;

mod day01;
mod day02;
mod day03;

fn main() {
    let mut runs = [
        day01::first,
        day01::second,
        day02::first,
        day02::second,
        day03::first,
        day03::second,
    ].iter().enumerate();

    while let Some((i, day)) = runs.next() {
        run_day(i / 2 + 1, day, runs.next().map(|a| a.1));
    }
}

fn run_day(day: usize, first: &fn(&str) -> i64, second: Option<&fn(&str) -> i64>) {
    let string = format!("input/{:02}.txt", day);
    let mut input = File::open(string).unwrap();

    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();
    println!("Day {:02} A: {}", day, first(&content));
    if let Some(second) = second {
        println!("Day {:02} B: {}", day, second(&content));
    }
}
