pub(crate) fn first(input: &str) -> i64 {
    let numbers = parse_data(input);

    for a in &numbers {
        for b in &numbers {
            if *a != *b && a + b == 2020 {
                return a * b;
            }
        }
    }
    unreachable!();
}

pub(crate) fn second(input: &str) -> i64 {
    let numbers = parse_data(input);

    for a in &numbers {
        for b in &numbers {
            for c in &numbers {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    unreachable!();
}

fn parse_data(input: &str) -> Vec<i64> {
    let numbers: Vec<i64> = input.split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|val| val.parse().unwrap())
        .collect();
    numbers
}
