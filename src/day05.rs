pub(crate) fn first(input: &str) -> i64 {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| get_seat_id(line))
        .max().unwrap() as i64
}

pub(crate) fn second(input: &str) -> i64 {
    let mut seats: Vec<i64> = input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| get_seat_id(line))
        .collect();

    seats.sort();

    let mut iter = seats.iter();

    let mut last_id = *iter.next().unwrap();
    loop {
        if let Some(id) = iter.next() {
            if *id == last_id + 2 {
                return id + 1;
            } else {
                last_id = *id;
            }
        }
    }
}

fn get_seat_id(line: &str) -> i64 {
    let row: String = line.chars()
        .take(7)
        .map(|c| if c == 'B' { '1' } else { '0' })
        .collect();
    let col: String = line.chars().skip(7)
        .map(|c| if c == 'R' { '1' } else { '0' })
        .collect();

    i64::from_str_radix(row.as_str(), 2).unwrap() * 8 + i64::from_str_radix(col.as_str(), 2).unwrap()
}

#[cfg(test)]
mod test {
    use super::get_seat_id;

    #[test]
    fn get_seat_id_1() {
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567)
    }

    #[test]
    fn get_seat_id_2() {
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119)
    }

    #[test]
    fn get_seat_id_3() {
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820)
    }
}
