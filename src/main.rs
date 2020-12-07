use std::io;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::cmp::PartialEq;

enum Direction{
    Up,
    Down,
}

#[derive(Debug)]
enum RangeOrSingle<T>{
    Range(RangeInclusive<T>),
    Single(T),
}
impl <T:PartialEq> PartialEq for RangeOrSingle<T>{
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (RangeOrSingle::Single(a), RangeOrSingle::Single(b)) => a==b,
            (RangeOrSingle::Range(a), RangeOrSingle::Range(b)) => a==b,
            _ => false,
        }
    }
}

fn main() {
   let mut stream = BufReader::new(io::stdin());
   let result = compute(&mut stream);
   println!("result: {:?}", result);
}

fn compute<T: BufRead>(input: &mut T) -> usize{
   input.lines().map(|passport| {
       let line = passport.unwrap();
       seat_id(&line)
    }).max().unwrap()
}

fn seat_id(position: &str) -> usize {
   row(position) * 8 + column(position)
}

fn front_back(ch: char) -> Direction{
    if ch =='F'{
        Direction::Down
    } else if ch == 'B'{
        Direction::Up
    } else {
       panic!("incorrect back/front character");
    }
}

fn left_right(ch: char) -> Direction{
    if ch =='L'{
        Direction::Down
    } else if ch == 'R'{
        Direction::Up
    } else {
       panic!("incorrect left/right character {:?}", ch);
    }
}

fn row(position: &str) -> usize{
    let rows_in_airplane = RangeOrSingle::Range(RangeInclusive::new(0, 127));
    let result = position.chars().take(7).fold(rows_in_airplane, |range, ch|{
        let a = match range{
            RangeOrSingle::Single(v) => range,
            RangeOrSingle::Range(_) => select_half(range, front_back(ch)),
        };
        a
    });
    let result = select_half(result, Direction::Up);
    match result{
        RangeOrSingle::Single(v) => v,
        _ => panic!("incorrect result {:?}", result),
    }
}

fn column(position: &str) -> usize{
    let columns_in_airplane = RangeOrSingle::Range(RangeInclusive::new(0, 7));
    let result = position.chars().skip(7).take(3).fold(columns_in_airplane, |range, ch|{
        let a = match range{
            RangeOrSingle::Single(v) => range,
            RangeOrSingle::Range(_) => select_half(range, left_right(ch)),
        };
        a
    });
    let result = select_half(result, Direction::Up);
    match result{
        RangeOrSingle::Single(v) => v,
        _ => panic!("incorrect result {:?}", result),
    }
}

fn select_half(range: RangeOrSingle<usize>, direction: Direction) -> RangeOrSingle<usize>{
    match range{
        RangeOrSingle::Single(v) => RangeOrSingle::Single(v),
        RangeOrSingle::Range(range) =>{
            if range.start() == range.end(){
                return RangeOrSingle::Single(*range.start());
            }
            let step = 1;
            let first_above_half = (((range.end() - range.start()) + step)/2) + (range.start() * step);
                match direction{
                    Direction::Up =>{
                        RangeOrSingle::Range(RangeInclusive::new(first_above_half, *range.end()))
                    },
                    Direction::Down =>{
                        RangeOrSingle::Range(RangeInclusive::new(*range.start(), first_above_half - step))
                    }
                }
        }
    }
}

#[test]
fn test_halving(){
    assert_eq!(
        RangeOrSingle::Range(RangeInclusive::new(8,11)),
        select_half(RangeOrSingle::Range(RangeInclusive::new(4,11)), Direction::Up)
        );
}

#[test]
fn test_row(){
    assert_eq!(44, row("FBFBBFFRLR"));
    assert_eq!(14, row("FFFBBBFRRR"));
    assert_eq!(102, row("BBFFBBFRLL"));
}

#[test]
fn test_column(){
    assert_eq!(7, column("BFFFBBFRRR"));
    assert_eq!(7, column("FFFBBBFRRR"));
    assert_eq!(4, column("BBFFBBFRLL"));
}

#[test]
fn test_seat_id(){
    assert_eq!(567, seat_id("BFFFBBFRRR"));
    assert_eq!(119, seat_id("FFFBBBFRRR"));
    assert_eq!(820, seat_id("BBFFBBFRLL"));
}
