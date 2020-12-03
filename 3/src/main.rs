use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let file_name  = "data.txt";   

    println!("Problem 1......{:?}", process(file_lines(file_name), 3,1));
    

    let r1d1 = process(file_lines(file_name), 1 , 1);
    println!("r1d1......{:?}", r1d1);

    let r3d1 = process(file_lines(file_name), 3,1);
    println!("r3d1......{:?}", r3d1);

    let r5d1 = process(file_lines(file_name), 5,1);
    println!("r5d1......{:?}", r5d1);

    let r7d1 = process(file_lines(file_name), 7,1);
    println!("r7d1......{:?}", r7d1);

    let r1d2 = process(file_lines(file_name), 1,2);
    println!("r1d2......{:?}", r1d2);

    println!("Problem 1......{:?}", r1d1 * r3d1 * r5d1* r7d1 * r1d2);
}

fn process(lines: Vec<String>, right: usize, down: usize) -> usize{
    let mut pos = 0;
    let tree = '#';

    return lines.iter()
        .step_by(down)
        .map(|s| { pos = mv_right(pos,right); char_at_pos(pos , s) })
        .filter(|c| c == &tree)
        .count();  
}

pub fn mv_right(start: usize, right: usize) -> usize {
    if start == 0 {
        return 1;
    }
    else {
        return start + right;
    };
}

pub fn char_at_pos(pos: usize, s: &str) -> char{

    //We need to get the Mod of the current value 
    let pos_to_grab = (pos-1) % s.len();
    //println!("char_at_pos......{:?}-{:?}-{:?}",pos, pos_to_grab, s);
    return s.chars().nth(pos_to_grab).unwrap()
}

fn file_lines(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let v: Vec<String> =  reader.lines().map(|s|s.unwrap()).collect();
    return v; 
}

#[test]
fn process_test() {
    let xmap: Vec<String> = vec![
        "...#".to_string(),
        ".#..".to_string(),
        "...#".to_string(),
        "####".to_string()
      ];
    
    assert_eq!(2, process(xmap.to_vec(), 1,1), "Right 1 Down 1");
    assert_eq!(0, process(xmap.to_vec(), 1,2), "Right 1 Down 2");
    assert_eq!(1, process(xmap.to_vec(), 2,1), "Right 2 Down 1");
}

#[test]
fn mv_right_test() {
    assert_eq!(1, mv_right(0,3));
    assert_eq!(4, mv_right(1,3));
}


#[test]
fn get_zero_char() {
    let s = "#..#..#.#.##....#.#........#..."; 
    assert_eq!('#', char_at_pos(1,s));
}
#[test]
fn get_last_char() {
    let s = "#..T"; 
    assert_eq!('T', char_at_pos(4,s));
}
#[test]
fn wrap_car() {
    let s = "A#A"; 
    assert_eq!('#', char_at_pos(5,s));
}
#[test]
fn wrap_car_long() {
    let s = "....c....."; 
    assert_eq!('c', char_at_pos(945,s));
}
