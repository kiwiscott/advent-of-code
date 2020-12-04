/// A SkiSlope here
pub struct SkiRun {
    /// The SkiSlope has a path through some trees
    run: Vec<String>,
}

impl SkiRun {
    pub fn new(run: Vec<String>) -> SkiRun {
        SkiRun { run: run }
    }

    /// Ski the Slope as best you can!
    ///
    /// Returns the number of trees hit during the run
    pub fn do_the_run(&self, right: usize, down: usize) -> usize {
        let mut pos = 0;
        let tree = '#';

        return self
            .run
            .to_vec()
            .iter()
            .step_by(down)
            .map(|s| {
                pos = mv_right(pos, right);
                char_at_pos(pos, s)
            })
            .filter(|c| c == &tree)
            .count();
    }
}

fn mv_right(start: usize, right: usize) -> usize {
    if start == 0 {
        return 1;
    } else {
        return start + right;
    };
}

fn char_at_pos(pos: usize, s: &str) -> char {
    //We need to get the Mod of the current value
    let pos_to_grab = (pos - 1) % s.len();
    //println!("char_at_pos......{:?}-{:?}-{:?}",pos, pos_to_grab, s);
    return s.chars().nth(pos_to_grab).unwrap();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn get_zero_char() {
        let s = "#..#..#.#.##....#.#........#...";
        assert_eq!('#', char_at_pos(1, s));
    }
    #[test]
    fn get_last_char() {
        let s = "#..T";
        assert_eq!('T', char_at_pos(4, s));
    }
    #[test]
    fn wrap_car() {
        let s = "A#A";
        assert_eq!('#', char_at_pos(5, s));
    }
    #[test]
    fn wrap_car_long() {
        let s = "....c.....";
        assert_eq!('c', char_at_pos(945, s));
    }
    #[test]
    fn mv_right_test() {
        assert_eq!(1, mv_right(0, 3));
        assert_eq!(4, mv_right(1, 3));
    }

    #[test]
    fn process_test() {
        let xmap: Vec<String> = vec![
            "...#".to_string(),
            ".#..".to_string(),
            "...#".to_string(),
            "####".to_string(),
        ];

        let sr = SkiRun::new(xmap);

        assert_eq!(2, sr.do_the_run(1, 1), "Right 1 Down 1");
        assert_eq!(0, sr.do_the_run(1, 2), "Right 1 Down 2");
        assert_eq!(1, sr.do_the_run(2, 1), "Right 2 Down 1");
    }
}
