use std::collections::HashMap;

use regex::Regex;

pub(crate) fn first(input: &str) -> i64 {
    input.split("\n\n")
        .filter(|line| is_valid(line))
        .count() as i64
}


pub(crate) fn second(input: &str) -> i64 {
    let hcl = Regex::new("#[0-9a-f]{6}").unwrap();
    let pid = Regex::new("[0-9]{9}").unwrap();

    input.split("\n\n")
        .filter(|line| is_valid(line))
        .filter(|line| {
            let map: HashMap<&str, &str> = line.split_ascii_whitespace()
                .map(|pair| pair.split_once(':'))
                .filter(|pair| pair.is_some())
                .map(|pair| pair.unwrap())
                .collect();

            map.get("byr").map(|v| v.parse::<i32>().map(|byr| (1920..=2002).contains(&byr)).unwrap_or(false)).unwrap_or(false) &&
                map.get("iyr").map(|v| (2010..=2020).contains(&v.parse::<i32>().unwrap())).unwrap_or(false) &&
                map.get("eyr").map(|v| (2020..=2030).contains(&v.parse::<i32>().unwrap())).unwrap_or(false) &&
                map.get("hgt").map(|v|
                    v.strip_suffix("cm").map(|hgt| hgt.parse::<i32>().map(|h| (150..=193).contains(&h)).unwrap_or(false)).unwrap_or(false) ||
                        v.strip_suffix("in").map(|hgt| hgt.parse::<i32>().map(|h| (59..=76).contains(&h)).unwrap_or(false)).unwrap_or(false)
                ).unwrap_or(false) &&
                map.get("hcl").map(|v| hcl.is_match(v)).unwrap_or(false) &&
                map.get("ecl").map(|v| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v)).unwrap_or(false) &&
                map.get("pid").map(|v| v.len() == 9 && pid.is_match(v)).unwrap_or(false)
        })
        .count() as i64
}


fn is_valid(passport: &str) -> bool {
    let keys: Vec<&str> = passport.split_ascii_whitespace()
        .map(|line| line.split_once(':').unwrap().0)
        .collect();
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|key| keys.contains(key))&&
        keys.len() == 7 || keys.len() == 8
}

#[cfg(test)]
mod test {
    use super::{is_valid, second};

    #[test]
    fn is_valid_1() {
        assert_eq!(is_valid("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"), true)
    }


    #[test]
    fn is_valid_2() {
        assert_eq!(is_valid("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929"), false)
    }

    #[test]
    fn is_valid_3() {
        assert_eq!(is_valid("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm"), true)
    }

    #[test]
    fn is_valid_4() {
        assert_eq!(is_valid("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in"), false)
    }

    #[test]
    fn second_invalid_1() {
        assert_eq!(second("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"), 0);
    }

    #[test]
    fn second_invalid_2() {
        assert_eq!(second("iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946"), 0);
    }

    #[test]
    fn second_invalid_3() {
        let input = "hcl:dab227
        iyr:2012
        ecl:brn
        hgt:182cm
        pid:021572410
        eyr:2020
        byr:1992
        cid:277";
        assert_eq!(second(input), 0);
    }

    #[test]
    fn second_invalid_4() {
        assert_eq!(second("hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007"), 0);
    }

    #[test]
    fn second_invalid_all(){
        assert_eq!(second("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"),0)
    }

    #[test]
    fn second_valid_1() {
        assert_eq!(second("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f"), 1);
    }

    #[test]
    fn second_valid_2() {
        assert_eq!(second("eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"), 1);
    }

    #[test]
    fn second_valid_3() {
        assert_eq!(second("hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022"), 1);
    }

    #[test]
    fn second_valid_4() {
        assert_eq!(second("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"), 1);
    }

    #[test]
    fn second_valid_all(){
        assert_eq!(second("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),4)
    }
}
