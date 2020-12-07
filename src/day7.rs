use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub fn solve_part1(rules: &Vec<(String, Vec<(usize, String)>)>) -> usize {
    let mut found = HashSet::new();
    find_rules_for_bag(&"shiny gold".to_string(), rules, &mut found);
    found.len()
}

fn find_rules_for_bag(
    bag: &String,
    rules: &Vec<(String, Vec<(usize, String)>)>,
    already_found: &mut HashSet<String>,
) {
    rules
        .iter()
        .filter(|(_, value)| value.iter().any(|(_, other_bag)| bag == other_bag))
        .for_each(|(key, _)| {
            if !already_found.contains(key) {
                already_found.insert(key.clone());
                find_rules_for_bag(key, rules, already_found);
            }
        });
}

pub fn solve_part2(rules: &Vec<(String, Vec<(usize, String)>)>) -> usize {
    find_total_bags_for_bag(&"shiny gold".to_string(), rules) - 1
}

fn find_total_bags_for_bag(bag: &String, rules: &Vec<(String, Vec<(usize, String)>)>) -> usize {
    1 + rules
        .iter()
        .find(|(key, _)| key == bag)
        .unwrap()
        .1
        .iter()
        .map(|(count, bag)| count * find_total_bags_for_bag(bag, rules))
        .sum::<usize>()
}

pub fn parse_part1(input: &str) -> Vec<(String, Vec<(usize, String)>)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<count>\d+) (?P<color>.+?) bags?").unwrap();
    }

    input
        .lines()
        .map(
            |line| match line.split(" bags contain ").collect::<Vec<_>>().as_slice() {
                &[bag, rule] => (
                    bag.to_owned(),
                    RE.captures_iter(rule)
                        .map(|capture| {
                            (
                                capture.name("count").unwrap().as_str().parse().unwrap(),
                                capture.name("color").unwrap().as_str().to_owned(),
                            )
                        })
                        .collect::<Vec<(usize, String)>>(),
                ),
                _ => panic!(),
            },
        )
        .collect()
}

pub fn part1() {
    let mut file = File::open("input/2020/day7.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&parse_part1(&input)));
}

pub fn part2() {
    let mut file = File::open("input/2020/day7.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part2(&parse_part1(&input)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_part1_example() {
        assert_eq!(solve_part1(&parse_part1("light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.")), 4);
    }

    #[test]
    fn solve_part2_example() {
        assert_eq!(solve_part2(&parse_part1("light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.")), 32);
    }

    #[test]
    fn parse_part1_example() {
        let expected = vec![
            (
                "light red".to_string(),
                vec![
                    (1, "bright white".to_string()),
                    (2, "muted yellow".to_string()),
                ],
            ),
            (
                "dark orange".to_string(),
                vec![
                    (3, "bright white".to_string()),
                    (4, "muted yellow".to_string()),
                ],
            ),
            (
                "bright white".to_string(),
                vec![(1, "shiny gold".to_string())],
            ),
            (
                "muted yellow".to_string(),
                vec![(2, "shiny gold".to_string()), (9, "faded blue".to_string())],
            ),
            (
                "shiny gold".to_string(),
                vec![
                    (1, "dark olive".to_string()),
                    (2, "vibrant plum".to_string()),
                ],
            ),
            (
                "dark olive".to_string(),
                vec![
                    (3, "faded blue".to_string()),
                    (4, "dotted black".to_string()),
                ],
            ),
            (
                "vibrant plum".to_string(),
                vec![
                    (5, "faded blue".to_string()),
                    (6, "dotted black".to_string()),
                ],
            ),
            ("faded blue".to_string(), vec![]),
            ("dotted black".to_string(), vec![]),
        ];

        assert_eq!(parse_part1("light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags."),
            expected
        );
    }
}
