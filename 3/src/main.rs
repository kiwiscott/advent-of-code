use ski_resort::file_helper;
use ski_resort::maze::SkiRun;

fn main() {
    let file_name = "data.txt";

    let lines = file_helper::all_lines(file_name);

    let sr = SkiRun::new(lines);

    println!("Problem 1......{:?}", sr.do_the_run(3, 1));

    let r1d1 = sr.do_the_run(1, 1);
    println!("r1d1......{:?}", r1d1);

    let r3d1 = sr.do_the_run(3, 1);
    println!("r3d1......{:?}", r3d1);

    let r5d1 = sr.do_the_run(5, 1);
    println!("r5d1......{:?}", r5d1);

    let r7d1 = sr.do_the_run(7, 1);
    println!("r7d1......{:?}", r7d1);

    let r1d2 = sr.do_the_run(1, 2);
    println!("r1d2......{:?}", r1d2);

    println!("Problem 2......{:?}", r1d1 * r3d1 * r5d1 * r7d1 * r1d2);
}
