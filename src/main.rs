use std::io;
use std::io::{BufRead, BufReader};

fn main() {
   let stream = BufReader::new(io::stdin());
   let lines: Vec<i32> = stream.lines().map(|l|{
        l.unwrap().parse::<i32>().unwrap()
   }).collect();

   let result = compute(lines);
   println!("result: {}", result.expect("no solution found"));
}

fn compute(input: Vec<i32>) -> Option<i32>{
   for (i, number) in input.iter().enumerate(){
       for number2 in input.iter().skip(i+1){
           if number + number2 == 2020{
               return Some(number * number2)
           }
       }
   }
   return None
}

#[test]
fn test_input(){
    assert_eq!(
        Some(514579),
        compute(vec![
             1721,
             979,
             366,
             299,
             675,
             1456
        ])
    )
}
