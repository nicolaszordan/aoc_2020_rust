use regex::Regex;
use std::fs::File;
use std::io::Read;

#[derive(Clone, PartialEq, Debug)]
struct Credentials {
    byr: Option<String>, // birth year
    iyr: Option<String>, // issue year
    eyr: Option<String>, // expiration year
    hgt: Option<String>, // height
    hcl: Option<String>, // hair color
    ecl: Option<String>, // eye color
    pid: Option<String>, // password id
    cid: Option<String>, // country id
}

fn solve_part1(passports: &[Credentials]) -> usize {
    passports
        .iter()
        .filter(|passport| {
            passport.byr.is_some()
                && passport.iyr.is_some()
                && passport.eyr.is_some()
                && passport.hgt.is_some()
                && passport.hcl.is_some()
                && passport.ecl.is_some()
                && passport.pid.is_some()
            // country id aren't important for now
        })
        .count()
}

fn solve_part2(passports: &[Credentials]) -> usize {
    passports
        .iter()
        .filter(|passport| {
            (passport.byr.is_some() && byr_is_valid(&passport.byr.as_ref().unwrap()))
                && (passport.iyr.is_some() && iyr_is_valid(&passport.iyr.as_ref().unwrap()))
                && (passport.eyr.is_some() && eyr_is_valid(&passport.eyr.as_ref().unwrap()))
                && (passport.hgt.is_some() && hgt_is_valid(&passport.hgt.as_ref().unwrap()))
                && (passport.hcl.is_some() && hcl_is_valid(&passport.hcl.as_ref().unwrap()))
                && (passport.ecl.is_some() && ecl_is_valid(&passport.ecl.as_ref().unwrap()))
                && (passport.pid.is_some() && pid_is_valid(&passport.pid.as_ref().unwrap()))
            // country id aren't important for now
        })
        .count()
}

fn parse_part1(input: &str) -> Vec<Credentials> {
    input
        .split("\n\n")
        .map(|chunk| {
            let mut ret = Credentials {
                byr: None,
                iyr: None,
                eyr: None,
                hgt: None,
                hcl: None,
                ecl: None,
                pid: None,
                cid: None,
            };
            for credential in chunk.split_whitespace() {
                let mut credential = credential.split(':');
                match credential.next().unwrap() {
                    "byr" => ret.byr = Some(credential.next().unwrap().to_owned()),
                    "iyr" => ret.iyr = Some(credential.next().unwrap().to_owned()),
                    "eyr" => ret.eyr = Some(credential.next().unwrap().to_owned()),
                    "hgt" => ret.hgt = Some(credential.next().unwrap().to_owned()),
                    "hcl" => ret.hcl = Some(credential.next().unwrap().to_owned()),
                    "ecl" => ret.ecl = Some(credential.next().unwrap().to_owned()),
                    "pid" => ret.pid = Some(credential.next().unwrap().to_owned()),
                    "cid" => ret.cid = Some(credential.next().unwrap().to_owned()),
                    _ => panic!(),
                }
            }
            ret
        })
        .collect()
}

fn byr_is_valid(input: &str) -> bool {
    match input.parse::<u32>() {
        Ok(byr) => byr >= 1920 && byr <= 2002,
        Err(_) => false,
    }
}

fn iyr_is_valid(input: &str) -> bool {
    match input.parse::<u32>() {
        Ok(iyr) => iyr >= 2010 && iyr <= 2020,
        Err(_) => false,
    }
}

fn eyr_is_valid(input: &str) -> bool {
    match input.parse::<u32>() {
        Ok(eyr) => eyr >= 2020 && eyr <= 2030,
        Err(_) => false,
    }
}

fn hgt_is_valid(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([0-9]{2,3})(in|cm)$").unwrap();
    }
    match RE.captures(input) {
        Some(captures) => {
            let height = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            if captures.get(2).unwrap().as_str() == "in" {
                height >= 59 && height <= 76
            } else {
                height >= 150 && height <= 193
            }
        }
        None => false,
    }
}

fn hcl_is_valid(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(input)
}

fn ecl_is_valid(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }
    RE.is_match(input)
}

fn pid_is_valid(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
    }
    RE.is_match(input)
}

pub fn part1() {
    let mut file = File::open("input/2020/day4.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&parse_part1(&input)));
}

pub fn part2() {
    let mut file = File::open("input/2020/day4.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part2(&parse_part1(&input)));
}

mod test {
    use super::*;

    #[test]
    fn solve_part1_example() {
        assert_eq!(
            solve_part1(&[
                Credentials {
                    ecl: Some(String::from("gry")),
                    pid: Some(String::from("860033327")),
                    eyr: Some(String::from("2020")),
                    hcl: Some(String::from("#fffffd")),
                    byr: Some(String::from("1937")),
                    iyr: Some(String::from("2017")),
                    cid: Some(String::from("147")),
                    hgt: Some(String::from("183cm")),
                },
                Credentials {
                    iyr: Some(String::from("2013")),
                    ecl: Some(String::from("amb")),
                    cid: Some(String::from("350")),
                    eyr: Some(String::from("2023")),
                    pid: Some(String::from("028048884")),
                    hcl: Some(String::from("#cfa07d")),
                    byr: Some(String::from("1929")),
                    hgt: None,
                },
                Credentials {
                    hcl: Some(String::from("#ae17e1")),
                    iyr: Some(String::from("2013")),
                    eyr: Some(String::from("2024")),
                    ecl: Some(String::from("brn")),
                    pid: Some(String::from("760753108")),
                    byr: Some(String::from("1931")),
                    hgt: Some(String::from("179cm")),
                    cid: None,
                },
                Credentials {
                    hcl: Some(String::from("#cfa07d")),
                    eyr: Some(String::from("2025")),
                    pid: Some(String::from("166559648")),
                    iyr: Some(String::from("2011")),
                    ecl: Some(String::from("brn")),
                    hgt: Some(String::from("59in")),
                    cid: None,
                    byr: None,
                }
            ]),
            2
        );
    }

    #[test]
    fn parse_part1_example() {
        assert_eq!(parse_part1("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in"),
            vec![Credentials{
                ecl: Some(String::from("gry")),
                pid: Some(String::from("860033327")),
                eyr: Some(String::from("2020")),
                hcl: Some(String::from("#fffffd")),
                byr: Some(String::from("1937")),
                iyr: Some(String::from("2017")),
                cid: Some(String::from("147")),
                hgt: Some(String::from("183cm")),
            }, Credentials{
                iyr: Some(String::from("2013")),
                ecl: Some(String::from("amb")),
                cid: Some(String::from("350")),
                eyr: Some(String::from("2023")),
                pid: Some(String::from("028048884")),
                hcl: Some(String::from("#cfa07d")),
                byr: Some(String::from("1929")),
                hgt: None,
            }, Credentials{
                hcl: Some(String::from("#ae17e1")),
                iyr: Some(String::from("2013")),
                eyr: Some(String::from("2024")),
                ecl: Some(String::from("brn")),
                pid: Some(String::from("760753108")),
                byr: Some(String::from("1931")),
                hgt: Some(String::from("179cm")),
                cid: None,
            }, Credentials{
                hcl: Some(String::from("#cfa07d")),
                eyr: Some(String::from("2025")),
                pid: Some(String::from("166559648")),
                iyr: Some(String::from("2011")),
                ecl: Some(String::from("brn")),
                hgt: Some(String::from("59in")),
                cid: None,
                byr: None,
            }]
        );
    }

    #[test]
    fn solve_part2_invalid_examples() {
        assert_eq!(solve_part2(&parse_part1("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007")), 0);
    }

    #[test]
    fn solve_part2_valid_examples() {
        assert_eq!(solve_part2(&parse_part1("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719")), 4);
    }
}
