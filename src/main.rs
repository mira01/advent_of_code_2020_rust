use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Policy{
    lower_bound: usize,
    upper_bound: usize,
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
        lower_bound: split[0].parse::<usize>().unwrap(),
        upper_bound: split[1].parse::<usize>().unwrap(),
        character: split[2].parse::<char>().unwrap()

    }, split[4].into())
}

fn complies(line: &(Policy, String)) -> bool{
   let (Policy{lower_bound, upper_bound, character}, password) = line;
   let count = password.chars()
       .filter(|ch| &ch == &character)
       .count();
   (lower_bound..&(upper_bound+1)).contains(&&count)
}

fn compute< I>(stream: I) -> usize
    where I: Iterator<Item=String>{
    stream.map(|l| parse_line(l))
        .filter(|i| complies(i))
        .count()
}

#[test]
fn test_input(){
    assert_eq!(2,
        compute(vec![
            "1-3 a: abcde".into(),
            "1-3 b: cdefg".into(),
            "2-9 c: ccccccccc".into(),
        ].into_iter())
    )
}
