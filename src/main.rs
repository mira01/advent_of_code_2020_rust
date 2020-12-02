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
       for (j, number2) in input.iter().skip(i+1).enumerate(){
           for number3 in input.iter().skip(j+1){
               if number + number2 + number3 == 2020{
                   return Some(number * number2 * number3)
               }
           }
       }
   }
   return None
}

#[test]
fn test_input(){
    assert_eq!(
        Some(241861950),
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
