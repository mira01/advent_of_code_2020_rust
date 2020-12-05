use std::io;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::cmp::max;

fn slopes() -> Vec<(usize, usize)> {
    vec![
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2)
    ]
}

fn main() {
   let stream = BufReader::new(io::stdin());
   let result = compute(&slopes(), stream.lines().map(|l| l.unwrap()));
   println!("result: {:?}", result);
}

fn compute<I>(slopes: &Vec<(usize, usize)>, lines: I) -> usize
where I: Iterator<Item=String> {
    let arr = vec![0; slopes.len()];
    lines.enumerate()
        .fold(arr, |arr, (line_no, line)|{
            let indexes = indexes(slopes, line_no);
            println!("indexes: {:?}", indexes);
            let trees = get_from_indexes(indexes, line);
            println!("trees: {:?}", trees);
            let sumvec = sum_vec(arr, trees);
            println!("sumvec: {:?}", sumvec);
            sumvec
    })
    .iter()
    .inspect(|x|{println!("inspecting {:?}", x)})
    .fold(1, |acc, x| { x * acc})
}

fn get_from_indexes(indexes: Vec<Option<usize>>, line: String) -> Vec<usize>{
    println!("line: {:?}", &line);
    let mut chars = line.chars()
                    .cycle();
    let mut vec = vec![];
    let mut prev_index = 1;
    let mut prev_item = 0;
    for index in indexes{
        println!("index: {:?}", index);
        if let Some(index) = index{
            if index == prev_index{
                vec.push(prev_item);
            } else{
                let iterator_index = max(((index as isize) - (prev_index as isize)-1), 0) as usize;
                println!("iterator_index: {:?}", iterator_index);
                let ch = ((chars.nth(iterator_index) == Some('#')) as usize);
                println!("ch: {:?}", ch);
                vec.push(ch);
                prev_index = index;
                prev_item = ch;
            }
        } else {
            prev_item = 0;
            vec.push(0);
        }
    }
    vec
}

fn indexes(paths: &Vec<(usize, usize)>, line_no: usize) -> Vec<Option<usize>>{
    paths.iter()
        .map(|(right, down)|{
            if line_no % down != 0{
                None
            }else{
               Some(line_no * right)
            }
        }).collect()
}

fn sum_vec<T: Add<Output=T> + Copy>(a: Vec<T>, b:Vec<T>) -> Vec<T>{
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(a, b)| *a + *b).collect()
}

#[test]
fn t_indexes(){
    let indexes = vec![Some(1), Some(3), Some(5), Some(7), None];
    assert_eq!(
        vec![0,0,0,0,0] ,
        get_from_indexes(indexes, "#...#...#..".into())
    );
}


#[test]
fn vectors(){
    assert_eq!(
        vec![10, 9, 8],
        sum_vec(vec![5,1,2], vec![5,8,6])
    );
}

#[test]
fn test_input(){
    assert_eq!(
        336,
        compute(&slopes(), vec![
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
