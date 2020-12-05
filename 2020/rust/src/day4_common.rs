extern crate regex;

use regex::Regex;

pub trait ValidDocCounter {
    fn validate_doc(&self, doc: &Vec<(&str, &str)>, expected_values: &Vec<&str>) -> usize;

    fn extract_fields<'a>(&self, row: &'a str) -> Vec<(&'a str, &'a str)> {
        let re = Regex::new(r"(\w+):([^ ]+)").unwrap();
        re.captures_iter(row).flat_map(|cap|
            match (cap.get(1), cap.get(2)) {
                (Some(field), Some(value)) => Some((field.as_str(), value.as_str())),
                _ => None,
            }
        ).collect()
    }


    fn number_valid_passports<'a>(&self, lines: &Vec<String>, expected_values: &Vec<&'a str>) -> usize {
        let mut count: usize = 0;
        let mut doc = vec![];
        let mut lines_iter = lines.iter();
        loop {
            match lines_iter.next() {
                Some(line) => {
                    if line.is_empty() {
                        count += self.validate_doc(&doc, expected_values);
                        doc = vec![];
                    } else {
                        self.extract_fields(line).iter().for_each(|item| doc.push(*item));
                    }
                }
                None => {
                    count += self.validate_doc(&doc, expected_values);
                    break;
                }
            }
        }
        count
    }
}

// #[test]
// fn extract_fields_test() {
//     let row = "iyr:2010 ecl:gry hgt:181cm pid:591597745 byr:1920 hcl:#6b5442 eyr:2029 cid:123";
//     let actuals = extract_fields(row);
//
//     vec!["iyr", "ecl", "hgt", "pid", "byr", "hcl", "eyr", "cid"]
//         .iter()
//         .zip(actuals)
//         .for_each(|(expected, actual)| {
//             assert_eq!(*expected, actual)
//         });
// }
//
// #[test]
// fn valid_doc_test() {
//     let valid = vec!["aaa", "bbb", "ccc"];
//     assert_eq!(0, validate_doc(&vec!["aaa", "bbb"], &valid));
//     assert_eq!(0, validate_doc(&vec!["aaa"], &valid));
//     assert_eq!(1, validate_doc(&vec!["aaa", "bbb", "ccc"], &valid));
//     assert_eq!(1, validate_doc(&vec!["aaa", "ddd", "bbb", "ccc"], &valid));
// }
