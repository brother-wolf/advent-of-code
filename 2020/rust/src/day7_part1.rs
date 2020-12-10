use std::io;

use advent_of_code_2020_rust::utils::file::lines_from_file;
use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let lines = lines_from_file("data/day7.txt")?;
    let mut bags = Baggage::<'static>::new(&lines);
    let bag = "shiny gold";
    println!("number of bags leading to {}: {}", bag, bags.count_bags_leading_to_type(bag));
    Ok(())
}

struct Baggage<'a> {
    rules: HashMap<&'a str, Vec<(usize, &'a str)>>,
    shortcutting: HashMap<String, usize>,
}

impl <'a> Baggage<'a> {
    pub fn new(lines: &Vec<String>) -> Baggage {
        let mut rules = HashMap::new();
        let shortcutting = HashMap::new();

        lines.iter().for_each(|line| {
            Baggage::parse_rules(line, &mut rules);
        });
        Baggage {rules, shortcutting }
    }

    fn parse_rules(sentence: &'a str, rules: &mut HashMap<&'a str, Vec<(usize, &'a str)>>) -> bool {
        let mut x = sentence.split(" bags contain ");
        let key = x.next().unwrap();
        assert!(!rules.contains_key(key));
        let contents = x.next().unwrap();
        let re = Regex::new(r#"(\d+) ([\w\s]+) bag"#).unwrap();
        let lists = re.captures_iter(contents).flat_map(|cap| {
            match (cap.get(1), cap.get(2)) {
                (Some(count), Some(colour)) => Some((usize::from_str(count.as_str()).unwrap(), colour.as_str())),
                _ => None,
            }
        }).collect::<Vec<(usize, &str)>>();
        let option = rules.insert(key, lists);
        option.is_some()
    }

    fn node_walker(&mut self, node: &str, acc: usize, description: &str) -> usize {
        let total = if self.shortcutting.contains_key(node) {
            *self.shortcutting.get(node).unwrap()
        } else {
            let this_acc = if node == description { 1 } else { 0 };
            let vec = self.rules.clone().get(node).unwrap().iter().flat_map(|(c, sn)| {
                (0..*c).map(|_it| {
                    self.node_walker(sn, this_acc, description)
                }).collect::<Vec<usize>>()
            }).collect_vec();

            let node_total = vec.iter().sum::<usize>() + this_acc;
            self.shortcutting.insert(String::from(node), node_total);
            node_total
        };
        acc + total
    }

    pub fn count_bags_leading_to_type(&mut self, description: &str) -> usize {
        self.shortcutting = HashMap::new();
        let keys = self.rules.keys().map(|k| *k).filter(|k| *k != description).collect::<Vec<&str>>();
        keys.iter().map(|&node| {
            (self.node_walker(&node, 0, description) > 0) as usize
        }).sum()
    }
}

#[test]
fn count_bags_of_type_test() {
    let mut rules = HashMap::new();
    rules.insert("muted yellow", vec![(1, "shiny gold"), (1, "faded blue")]);
    rules.insert("shiny gold", vec![]);
    rules.insert("faded blue", vec![]);
    rules.insert("pale pink", vec![(2, "faded blue")]);
    let mut baggage = Baggage{rules, shortcutting: Default::default()};
    // assert_eq!(1, baggage.count_bags_of_type("shiny gold"));
    assert_eq!(2, baggage.count_bags_leading_to_type("faded blue"));
}

#[test]
fn example1_test() -> io::Result<()> {
    let lines = lines_from_file("data/day7-example1.txt")?;
    let mut bags = Baggage::<'static>::new(&lines);
    assert_eq!(4, bags.count_bags_leading_to_type("shiny gold"));
    assert_eq!(7, bags.count_bags_leading_to_type("faded blue"));
    Ok(())
}



#[test]
fn parser_rules_test() {
    let s = vec!["light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string()];
    let baggage = Baggage::new(&s);

    let actual = baggage.rules.get("light red").unwrap();
    let expected = vec![(1, "bright white"), (2, "muted yellow")];
    assert_vec_eq(expected, actual);
}

#[cfg(test)]
fn assert_vec_eq(expected: Vec<(usize, &str)>, actual: &Vec<(usize, &str)>) {
    expected.iter().enumerate().for_each(|(idx, expect)| {
        assert_eq!(expect.0, actual.get(idx).unwrap().0);
        assert_eq!(expect.1, actual.get(idx).unwrap().1);
    })
}
