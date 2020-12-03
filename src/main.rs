use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Policy{
    position1: usize,
    position2: usize,
    character: char,
}

fn main() {
   let stream = BufReader::new(io::stdin());
   let result = compute(
       stream.lines()
       .map(|l| l.unwrap())
   );
   println!("result: {:?}", result);
}

fn parse_line(line: String) -> (Policy, String){
    let split: Vec<&str> = line.split(|c| c == '-' || c== ' ' || c== ':').collect();
    (Policy{
        position1: split[0].parse::<usize>().unwrap(),
        position2: split[1].parse::<usize>().unwrap(),
        character: split[2].parse::<char>().unwrap()

    }, split[4].into())
}

fn complies(line: &(Policy, String)) -> bool{
   let (Policy{position1, position2, character}, password) = line;
   let letters: Vec<char> = password.chars().collect();
   let letter1 = letters.get(position1 -1);
   let letter2 = letters.get(position2 -1);
   match (letter1, letter2) {
        (Some(first), Some(second)) if first == character && second != character => true,
        (Some(first), Some(second)) if first != character && second == character => true,
        _ => false,
   }
}

fn compute< I>(stream: I) -> usize
    where I: Iterator<Item=String>{
    stream.map(|l| parse_line(l))
        .filter(|i| complies(i))
        .count()
}

#[test]
fn test_input(){
    assert_eq!(1,
        compute(vec![
            "1-3 a: abcde".into(),
            "1-3 b: cdefg".into(),
            "2-9 c: ccccccccc".into(),
        ].into_iter())
    )
}
