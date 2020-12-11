use std::convert::TryFrom;
use std::fmt;

use aoc::common::*;

const COVID_ADJ_THRESHOLD: u8 = 4;

fn main() {
    let data: Vec<String> = read_file("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let mut ferry = Ferry::from(data);
    ferry.covid_adj_threshold = 4;
    while ferry.shuffle() {
        ()
    }
    println!("Problem 1: {:?}", ferry.occupied_seats());

    let data: Vec<String> = read_file("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let mut ferry = Ferry::from(data);
    ferry.covid_adj_threshold = 5;
    ferry.max_distance = 100;
    while ferry.shuffle() {
        ()
    }
    println!("Problem 2: {:?}", ferry.occupied_seats());
}

#[derive(Debug)]
struct Ferry {
    covid_adj_threshold: u8,
    rows: Vec<Vec<SeatState>>,
    already_shuffled: bool,
    max_distance: i32,
}

impl Ferry {
    /// Returns a bool indicating if people seats states changed.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    fn shuffle(&mut self) -> bool {
        let mut new_rows = Vec::new();
        let mut were_there_changes = false;

        for (row_index, row) in self.rows.iter().enumerate() {
            let mut new_seats = Vec::new();
            for (seat_index, seat) in row.iter().enumerate() {
                let (changed, new_state) =
                    self.new_state(*seat, (row_index as isize) + 1, (seat_index as isize) + 1);

                if changed {
                    were_there_changes = true;
                }
                new_seats.push(new_state);
            }
            new_rows.push(new_seats);
        }
        self.rows = new_rows;
        self.already_shuffled = true;
        were_there_changes
    }

    fn occupied_seats(&self) -> usize {
        self.rows
            .iter()
            .map(|seats| {
                seats
                    .iter()
                    .filter(|seat| **seat == SeatState::Occupied)
                    .count()
            })
            .sum()
    }

    fn new_state(&self, state: SeatState, row: isize, seat: isize) -> (bool, SeatState) {
        /*
         - If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
         - If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
         - Otherwise, the seat's state does not change.
        */
        //println!("Checking Seat {:?} {:?} {:?}", state, row, seat);
        if state == SeatState::Floor {
            return (false, SeatState::Floor);
        }

        if !self.already_shuffled {
            return (true, SeatState::Occupied);
        }

        println!("{:?}",self.adjacent_occupied_seats(row, seat));

        if state == SeatState::Empty && self.adjacent_occupied_seats(row, seat) == 0 {
            return (true, SeatState::Occupied);
        }

        if state == SeatState::Occupied
            && self.adjacent_occupied_seats(row, seat) >= self.covid_adj_threshold
        {
            return (true, SeatState::Empty);
        }

        (false, state)
    }
    /*
    fn adjacent_seat_check(&self, row: isize, seat: isize, direction: Direction) -> bool {
        let (row_inc, seat_inc) = match direction {
            Direction::Left => (0, -1),
            Direction::DiagonalUpLeft => (-1, -1),
            Direction::Up => (-1, 0),
            Direction::DiagonalUpRight => (-1, 1),
            Direction::Right => (0, 1),
            Direction::DiagonalDownRight => (1, 1),
            Direction::Down => (1, 0),
            Direction::DiagonalDownLeft => (1, -1),
        };

        let mut result = false;
        let mut breakme = false;

        let mut check_row = row;
        let mut check_seat = seat;
        let mut counter = 0;
        while !breakme && (counter < self.max_distance) {
            counter = counter + 1;

            check_row = check_row + row_inc;
            check_seat = check_seat + seat_inc;

            match self.seat_occupied(check_row, check_seat) {
                None => breakme = true,
                Some((true, SeatState::Occupied)) => {
                    result = true;
                    breakme = true;
                },
                Some((_, SeatState::Floor)) => {
                    breakme = false;
                }
                Some((false, SeatState::Empty)) => {
                    breakme = true;
                },
                _=> ()
            }
        }

        result
    }

    fn adjacent_occupied_seats(&self, row: isize, seat: isize) -> u8 {
        let mut occupied = 0;

        let directions = [
            Direction::Left,
            Direction::DiagonalUpLeft,
            Direction::Up,
            Direction::DiagonalUpRight,
            Direction::Right,
            Direction::DiagonalDownRight,
            Direction::Down,
            Direction::DiagonalDownLeft,
        ];

        for d in directions.iter() {
            match self.adjacent_seat_check(row, seat, *d) {
                true => occupied += 1,
                _ => (),
            }
        }

        return occupied;
    } */

    fn adjacent_occupied_seats(&self, row: isize, seat: isize) -> u8 {
        let mut neighbours = 0;

        let mut check_row = row;
        let mut check_seat = seat;

        for ri in [-1, 0, 1].iter() {
            for si in [-1, 0, 1].iter() {
                if *ri == 0 && *si == 0 {
                    continue;
                }

                if (check_row + ri) < 0 || (check_seat + si) < 0 {
                    continue;
                }

                let check_row = usize::try_from(check_row + ri).unwrap();
                let check_seat = usize::try_from(check_seat + si).unwrap();

                if check_row < self.rows.len()
                    && check_seat < self.rows[0].len()
                    && self.rows[check_row][check_seat] == SeatState::Occupied
                {
                    neighbours += 1;
                }
            }
        }

        return neighbours;
    }

    fn seat_occupied(&self, row: isize, seat: isize) -> Option<(bool, SeatState)> {
        if row <= 0 || seat <= 0 {
            return None; //seat and rows are zero indexed so this is off the front or the left
        }
        let zrow = usize::try_from(row - 1).unwrap();
        let zseat = usize::try_from(seat - 1).unwrap();

        match self.rows.get(zrow) {
            None => return None,
            Some(row) => match row.get(zseat) {
                Some(SeatState::Occupied) => Some((true, SeatState::Occupied)),
                Some(ss) => Some((false, *ss)),
                _ => None,
            },
        }
    }
}

impl fmt::Display for Ferry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = String::new();
        for r in &self.rows {
            r.iter().for_each(|ss| {
                formatted += &ss.to_string();
            });
            formatted += "\n";
        }
        write!(f, "{}", formatted)
    }
}

impl From<Vec<String>> for Ferry {
    fn from(lines: Vec<String>) -> Self {
        let mut rows: Vec<Vec<SeatState>> = vec![];

        for line in lines {
            let parsed = line
                .chars()
                .map(|c| SeatState::from(c))
                .collect::<Vec<SeatState>>();
            rows.push(parsed);
        }

        Ferry {
            rows: rows,
            covid_adj_threshold: COVID_ADJ_THRESHOLD,
            already_shuffled: false,
            max_distance: 1,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    DiagonalUpLeft,
    Up,
    DiagonalUpRight,
    Right,
    DiagonalDownRight,
    Down,
    DiagonalDownLeft,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SeatState {
    Floor,
    Occupied,
    Empty,
}

impl From<char> for SeatState {
    fn from(c: char) -> Self {
        match c {
            '.' => SeatState::Floor,
            '#' => SeatState::Occupied,
            'L' => SeatState::Empty,
            _ => panic!("Cannot parse bad data!!"),
        }
    }
}

impl fmt::Display for SeatState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeatState::Floor => write!(f, "."),
            SeatState::Occupied => write!(f, "#"),
            SeatState::Empty => write!(f, "L"),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn ferry_seats() {
        let s = vec![String::from("L.LL.LL.L#")];
        let expected = vec![
            SeatState::Empty,
            SeatState::Floor,
            SeatState::Empty,
            SeatState::Empty,
            SeatState::Floor,
            SeatState::Empty,
            SeatState::Empty,
            SeatState::Floor,
            SeatState::Empty,
            SeatState::Occupied,
        ];
        assert_eq!(expected, Ferry::from(s).rows[0]);
    }
    #[test]
    fn check_adjacent_count() {
        let s = vec![
            String::from("LLL"),
            String::from("LLL"),
            String::from("#L."),
        ];
        let ferry = Ferry::from(s);
        assert_eq!(0, ferry.adjacent_occupied_seats(1, 1), "Row 1 Seat 1");
        assert_eq!(0, ferry.adjacent_occupied_seats(1, 2), "Row 1 Seat 2");
        assert_eq!(0, ferry.adjacent_occupied_seats(1, 3), "Row 1 Seat 3");

        assert_eq!(1, ferry.adjacent_occupied_seats(2, 1), "Row 2 Seat 1");
        assert_eq!(1, ferry.adjacent_occupied_seats(2, 2), "Row 2 Seat 2");
        assert_eq!(0, ferry.adjacent_occupied_seats(2, 3), "Row 2 Seat 3");

        assert_eq!(0, ferry.adjacent_occupied_seats(3, 1), "Row 3 Seat 1");
        assert_eq!(1, ferry.adjacent_occupied_seats(3, 2), "Row 3 Seat 2");
        assert_eq!(0, ferry.adjacent_occupied_seats(3, 3), "Row 3 Seat 3");
    }
    #[test]
    fn first_shuffle_all_seats_occupied() {
        let s = vec![
            String::from("LLL"),
            String::from("LLL"),
            String::from("LLL"),
        ];
        let expected = vec![
            SeatState::Occupied,
            SeatState::Occupied,
            SeatState::Occupied,
        ];

        let mut ferry = Ferry::from(s);
        let changes = ferry.shuffle();
        //First Shuffle
        assert!(changes, "Expected Changes");
        assert_eq!(expected, ferry.rows[0], "Row 1");
        assert_eq!(expected, ferry.rows[1], "Row 2");
        assert_eq!(expected, ferry.rows[2], "Row 3");
    }
    #[test]
    fn second_shuffle_corners_occupied() {
        let s = vec![
            String::from("LLL"),
            String::from("LLL"),
            String::from("LLL"),
        ];
        let mut ferry = Ferry::from(s);
        ferry.shuffle();
        println!("{}", ferry.to_string());
        let changes = ferry.shuffle();
        println!("{}", ferry.to_string());

        println!("{:?}", ferry);

        //Second Shuffle
        assert!(changes, "Expected Changes");
        assert_eq!(
            ferry.rows[0],
            vec![SeatState::Occupied, SeatState::Empty, SeatState::Occupied],
            "Row 1"
        );
        assert_eq!(
            ferry.rows[1],
            vec![SeatState::Empty, SeatState::Empty, SeatState::Empty],
            "Row 2"
        );
        assert_eq!(
            ferry.rows[2],
            vec![SeatState::Occupied, SeatState::Empty, SeatState::Occupied],
            "Row 3"
        );
    }
    #[test]
    fn during_suffle_floors_dont_change() {
        let s = vec![
            String::from("L.L"),
            String::from("L.L"),
            String::from("L.L"),
        ];
        let mut ferry = Ferry::from(s);
        ferry.shuffle();
        //Second Shuffle
        let expected = vec![SeatState::Occupied, SeatState::Floor, SeatState::Occupied];

        assert_eq!(ferry.rows[0], expected, "Row 1");
        assert_eq!(ferry.rows[1], expected, "Row 2");
        assert_eq!(ferry.rows[2], expected, "Row 3");
    }
    #[test]
    fn run_with_sample_data() {
        let s = vec![
            String::from("L.LL.LL.LL"),
            String::from("LLLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLLL"),
            String::from("L.LLLLLL.L"),
            String::from("L.LLLLL.LL"),
        ];
        let mut ferry = Ferry::from(s);
        while ferry.shuffle() {
            println!("{}", ferry.to_string());
        }

        //From Expected Rsults
        let expected_results = vec![
            String::from("#.#L.L#.##"),
            String::from("#LLL#LL.L#"),
            String::from("L.#.L..#.."),
            String::from("#L##.##.L#"),
            String::from("#.#L.LL.LL"),
            String::from("#.#L#L#.##"),
            String::from("..L.L....."),
            String::from("#L#L##L#L#"),
            String::from("#.LLLLLL.L"),
            String::from("#.#L#L#.##"),
        ];
        let expected_ferry = Ferry::from(expected_results);
        for row in 0..10 {
            assert_eq!(ferry.rows[row], expected_ferry.rows[row], "Row {:?}", row);
        }
    }
    #[test]
    fn run_with_sample_data_round_2() {
        let s = vec![
            String::from("L.LL.LL.LL"),
            String::from("LLLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLLL"),
            String::from("L.LLLLLL.L"),
            String::from("L.LLLLL.LL"),
        ];

        let mut ferry = Ferry::from(s);
        ferry.covid_adj_threshold = 5;
        ferry.max_distance = 100;

        ferry.shuffle();
        ferry.shuffle();
        let expected = vec![
            String::from("#.LL.LL.L#"),
            String::from("#LLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLL#"),
            String::from("#.LLLLLL.L"),
            String::from("#.LLLLL.L#"),
        ];
        let expected_ferry = Ferry::from(expected);
        for row in 0..10 {
            assert_eq!(ferry.rows[row], expected_ferry.rows[row], "Row {:?}", row);
        }

        while ferry.shuffle() {
            println!("{}", ferry.to_string());
        }
        assert_eq!(26, ferry.occupied_seats(), "From Sample Data ");
    }

    #[test]
    fn seats_to_check_test() {
        let s = vec![
            String::from("LLL"),
            String::from("LLL"),
            String::from("LLL"),
        ];

        let mut ferry = Ferry::from(s);
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::Left));
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::DiagonalUpLeft));
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::Up));
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::DiagonalUpRight));
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::Right));
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::DiagonalDownRight));
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::Down));
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::DiagonalDownLeft));
    }

    #[test]
    fn seats_to_check_test_all() {
        let s = vec![
            String::from("###"),
            String::from("###"),
            String::from("###"),
        ];

        let mut ferry = Ferry::from(s);
        assert!(ferry.adjacent_seat_check(2, 2, Direction::Left));
        assert!(ferry.adjacent_seat_check(2, 2, Direction::DiagonalUpLeft));
        assert!(ferry.adjacent_seat_check(2, 2, Direction::Up));
        assert!(ferry.adjacent_seat_check(2, 2, Direction::DiagonalUpRight));
        assert!(ferry.adjacent_seat_check(2, 2, Direction::Right));
        assert!(ferry.adjacent_seat_check(2, 2, Direction::DiagonalDownRight));
        assert!(ferry.adjacent_seat_check(2, 2, Direction::Down));
        assert!(ferry.adjacent_seat_check(2, 2, Direction::DiagonalDownLeft));
    }
    #[test]
    fn seats_to_check_test_all_true() {
        let s = vec![
            String::from("###"),
            String::from("###"),
            String::from("###"),
        ];

        let mut ferry = Ferry::from(s);
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::Left));
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::DiagonalUpLeft));
        assert!(!ferry.adjacent_seat_check(1, 1, Direction::Up));
        assert!(!ferry.adjacent_seat_check(2, 3, Direction::DiagonalUpRight));
        assert!(!ferry.adjacent_seat_check(2, 3, Direction::Right));
        assert!(!ferry.adjacent_seat_check(2, 3, Direction::DiagonalDownRight));
        assert!(!ferry.adjacent_seat_check(3, 3, Direction::Down));
        assert!(!ferry.adjacent_seat_check(3, 3, Direction::DiagonalDownLeft));
    }
    #[test]
    fn seats_to_check_test_all_true_surrounded() {
        let s = vec![
            String::from("#####"),
            String::from("#...#"),
            String::from("#...#"),
            String::from("#...#"),
            String::from("#####"),
        ];

        let mut ferry = Ferry::from(s);
        ferry.max_distance = 5;
        let alld = [
            Direction::Left,
            Direction::DiagonalUpLeft,
            Direction::Up,
            Direction::DiagonalUpRight,
            Direction::Right,
            Direction::DiagonalDownRight,
            Direction::Down,
            Direction::DiagonalDownLeft,
        ];
        for d in alld.iter() {
            assert!(ferry.adjacent_seat_check(3, 3, *d), "{:?}", d);
        }
    }
    #[test]
    fn seats_to_check_test_all_extreme_true() {
        let s = vec![
            String::from("#####"),
            String::from("#...#"),
            String::from("#...#"),
            String::from("#...#"),
            String::from("#####"),
        ];

        let mut ferry = Ferry::from(s);
        ferry.max_distance = 100;
        assert!(ferry.adjacent_seat_check(2, 1, Direction::Right), "Right");
        assert!(ferry.adjacent_seat_check(2, 4, Direction::Left), "Left");
        assert!(ferry.adjacent_seat_check(1, 2, Direction::Down), "Up");
        assert!(ferry.adjacent_seat_check(5, 4, Direction::Up), "Down");

        assert!(
            ferry.adjacent_seat_check(1, 1, Direction::DiagonalDownRight),
            "DiagonalDownRight"
        );
        assert!(
            ferry.adjacent_seat_check(1, 5, Direction::DiagonalDownLeft),
            "DiagonalDownLeft"
        );
        assert!(
            ferry.adjacent_seat_check(5, 1, Direction::DiagonalUpRight),
            "DiagonalUpRight"
        );
        assert!(
            ferry.adjacent_seat_check(5, 5, Direction::DiagonalUpLeft),
            "DiagonalUpLeft"
        );
    }
}
