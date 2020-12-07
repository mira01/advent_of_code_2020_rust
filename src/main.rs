use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let mut stream = BufReader::new(io::stdin());
    let result = compute(&mut stream);
    println!("result: {:?}", result);
}

fn compute<T: BufRead>(input: &mut T) -> usize {
    PassportParser(input).filter(|p| p.valid()).count()
}

fn valid_number(lower: u16, upper: u16) -> Box<dyn Fn(&str) -> bool> {
    Box::new(move |value| match value.parse::<u16>() {
        Ok(number) => (lower..upper + 1).contains(&number),
        _ => false,
    })
}

#[derive(Debug)]
struct Passport(HashMap<String, String>);
impl Passport {
    pub fn valid(&self) -> bool {
        let required: Vec<(String, Box<dyn Fn(&str) -> bool>)> = vec![
            (
                "ecl".into(),
                Box::new(|v| {
                    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                        .iter()
                        .any(|color| color == &v)
                }),
            ),
            ("byr".into(), Box::new(|v| valid_number(1920, 2020)(v))),
            ("iyr".into(), Box::new(|v| valid_number(2010, 2020)(v))),
            ("eyr".into(), Box::new(|v| valid_number(2020, 2030)(v))),
            (
                "hgt".into(),
                Box::new(|v| {
                    if v.ends_with("cm") {
                        valid_number(150, 193)(&v[..(v.len() - 2)])
                    } else if v.ends_with("in") {
                        valid_number(59, 76)(&v[..(v.len() - 2)])
                    } else {
                        false
                    }
                }),
            ),
            (
                "hcl".into(),
                Box::new(|v| {
                    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                    re.is_match(v)
                }),
            ),
            (
                "pid".into(),
                Box::new(|v| {
                    let re = Regex::new(r"^[0-9]{9}$").unwrap();
                    re.is_match(v)
                }),
            ),
        ];
        required
            .iter()
            .all(|(field, validation)| match self.0.get(field) {
                None => false,
                Some(value) => validation(value),
            })
    }

    pub fn new(record: String) -> Self {
        let hm = record
            .split_whitespace()
            .map(|field| {
                let mut field_parts = field.split(':');
                let key = field_parts.next().expect("could not parse field").into();
                let value = field_parts.next().expect("could not parse field").into();
                (key, value)
            })
            .collect();
        Self(hm)
    }
}

struct PassportParser<'a, T: BufRead>(&'a mut T);
impl<T: BufRead> Iterator for PassportParser<'_, T> {
    type Item = Passport;

    fn next(&mut self) -> Option<Passport> {
        let mut record = String::new();
        for l in self.0.lines() {
            let l = l.unwrap();
            if l != "" {
                record.push_str(&l);
                record.push_str(" ");
            } else {
                break;
            }
        }
        if record != "" {
            Some(Passport::new(record.trim_end().to_string()))
        } else {
            None
        }
    }
}
#[test]
fn test_input() {
    let mut input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
        .as_bytes();
    assert_eq!(2, compute(&mut input))
}
