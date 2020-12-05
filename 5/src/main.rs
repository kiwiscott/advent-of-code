use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let _p = all_lines("data.txt").iter().map(|c| seat_id(&c)).max();

    println!("Problem 1 Max:{:?}", _p);

    let mut allocated_seats: Vec<u32> = all_lines("data.txt").iter().map(|c| seat_id(&c)).collect();
    allocated_seats.sort();

    let _free_seats: Vec<u32> = (8.._p.unwrap())
        .filter(|i| !allocated_seats.contains(i))
        .collect();

    println!("Problem 2 :{:?}", _free_seats);
    println!("Problem 2: Assumes some seats are missing so guess its back further");
}

pub fn all_lines(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let v: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    return v;
}

fn seat_id(code: &str) -> u32 {
    //Have to use floats here as there's no ceiling / floor function in 'integers'
    //and casting back and forth isn't a good idea. 
    
    let mut last: f32 = 127.0;
    let mut start: f32 = 0.0;

    let mut clast: f32 = 7.0;
    let mut cstart: f32 = 0.0;

    for c in code.chars() {
        match c {
            'F' => last = start + ((last - start) / 2.0).floor(),
            'B' => start = start + ((last - start) / 2.0).ceil(),

            'L' => clast = cstart + ((clast - cstart) / 2.0).floor(),
            'R' => cstart = cstart + ((clast - cstart) / 2.0).ceil(),

            _ => (),
        }
    }
    return (last * 8.0 + clast) as u32;
}

#[test]
fn testx() {
    assert_eq!(357, seat_id("FBFBBFFRLR"), "FBFBBFFRLR");
    assert_eq!(567, seat_id("BFFFBBFRRR"), "BFFFBBFRRR");
    assert_eq!(119, seat_id("FFFBBBFRRR"), "FFFBBBFRRR");
    assert_eq!(820, seat_id("BBFFBBFRLL"), "BBFFBBFRLL");
}
