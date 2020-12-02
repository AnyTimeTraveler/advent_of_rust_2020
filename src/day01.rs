pub(crate) fn first(input: String) -> u64 {
    let numbers: Vec<u64> = input.split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|val| val.parse().unwrap())
        .collect();

    for a in &numbers {
        for b in &numbers {
            if *a != *b && a + b == 2020 {
                return a * b;
            }
        }
    }
    unreachable!();
}

pub(crate) fn second(input: String) -> u64 {
    let numbers: Vec<u64> = input.split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|val| val.parse().unwrap())
        .collect();

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