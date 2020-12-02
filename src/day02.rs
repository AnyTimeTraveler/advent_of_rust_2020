use std::ops::Range;

use regex::Regex;

struct Data {
    range: Range<usize>,
    char: char,
    password: String,
}

pub(crate) fn first(input: &str) -> i64 {
    parse_data(input)
        .iter()
        .filter(|data|
            data.range.contains(
                &data.password.chars()
                    .filter(|char| *char == data.char)
                    .count()
            )
        )
        .count() as i64
}


pub(crate) fn second(input: &str) -> i64 {
    parse_data(input)
        .iter()
        .map(|data| (data.password.chars().nth(data.range.start - 1), data.password.chars().nth(data.range.end - 2), data.char))
        .filter(|(a, b, _)| a.is_some() && b.is_some())
        .filter(|(a, b, char)| a.unwrap() == *char || b.unwrap() == *char)
        .filter(|(a, b, char)| !(a.unwrap() == *char && b.unwrap() == *char))
        .count() as i64
}

fn parse_data(input: &str) -> Vec<Data> {
    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    input.split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(move |line| {
            let cap = re.captures_iter(line).next().unwrap();
            Data {
                range: (&cap[1]).parse::<usize>().unwrap()..(&cap[2]).parse::<usize>().unwrap() + 1,
                char: (&cap[3]).chars().next().unwrap(),
                password: (&cap[4]).to_string(),
            }
        })
        .collect()
}


mod test {
    use super::second;

    #[test]
    fn second_1() {
        assert_eq!(second(&"1-3 a: abcde".to_string()), 1)
    }

    #[test]
    fn second_2() {
        assert_eq!(second(&"1-3 b: cdefg".to_string()), 0)
    }

    #[test]
    fn second_3() {
        assert_eq!(second(&"2-9 c: ccccccccc".to_string()), 0)
    }
}
