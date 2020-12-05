use std::io;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn main() {
   let mut stream = BufReader::new(io::stdin());
   let result = compute(&mut stream);
   println!("result: {:?}", result);
}

fn compute<T: BufRead>(input: &mut T) -> usize{
    PassportParser(input).filter(|p|{
        p.valid()
    }).count()
}

#[derive(Debug)]
struct Passport(HashMap<String, String>);
impl Passport{
    pub fn valid(&self) -> bool{
       let required: Vec<String> = vec![
              "byr".into(),
              "iyr".into(),
              "eyr".into(),
              "hgt".into(),
              "hcl".into(),
              "ecl".into(),
              "pid".into()
       ];
       required.iter().all(|field|{
        self.0.contains_key(field)
       })
    }

    pub fn new(record: String) -> Self{
        let hm = record.split_whitespace().map(|field|{
            let mut field_parts = field.split(':');
            let key = field_parts.next().expect("could not parse field").into();
            let value = field_parts.next().expect("could not parse field").into();
            (key, value)
        }).collect();
        Self(hm)
    }

}

struct PassportParser<'a, T: BufRead>(&'a mut T);
impl <T: BufRead>Iterator for PassportParser<'_, T>{
    type Item = Passport;

    fn next(&mut self) -> Option<Passport>{
        let mut record = String::new();
        for l in self.0.lines(){
            let l = l.unwrap();
            if l != ""{
                record.push_str(&l);
                record.push_str(" ");
            }
            else{
                break;
            }
        }
        if record != ""{
            Some(Passport::new(record.trim_end().to_string()))
        }else{
            None
        }
     }
}
#[test]
fn test_input(){
    let mut input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in".as_bytes();
    assert_eq!(
        2,
        compute(&mut input)
    )
}
