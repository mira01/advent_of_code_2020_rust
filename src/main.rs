use std::io;
use std::io::{BufRead, BufReader};


fn main() {
   let stream = BufReader::new(io::stdin());
   let result = compute(stream.lines().map(|l| l.unwrap()));
   println!("result: {:?}", result);
}

fn compute<I>(lines: I) -> usize
where I: Iterator<Item=String>{
    let(sum, index) = lines
        .step_by(1)
        .fold((0, 0), |(sum, index), line|{
            let ch = line.chars()
                .cycle()
                .skip(index)
                .next()
                .unwrap();
            let sum = if ch == '#'{ sum +1 } else {sum};
            (sum, index + 3)
            }
        );
    sum
}

#[test]
fn test_input(){
    assert_eq!(
        7,
        compute(vec![
            "..##.......".into(),
            "#...#...#..".into(),
            ".#....#..#.".into(),
            "..#.#...#.#".into(),
            ".#...##..#.".into(),
            "..#.##.....".into(),
            ".#.#.#....#".into(),
            ".#........#".into(),
            "#.##...#...".into(),
            "#...##....#".into(),
            ".#..#...#.#".into()
        ].into_iter())
    )
}
