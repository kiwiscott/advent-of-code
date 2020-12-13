use aoc::common::*;

fn main() {
    let data: Vec<String> = read_file("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    problem1(&data);
    problem2(&data);
}

fn problem2(data: &Vec<String>) {
    let mut buses: Vec<(i64, i64)> = data[1]
        .split(',')
        .enumerate()
        .filter(|(_, f)| f != &"x")
        .map(|(i, b)| {
            let i = i as i64;
            let b = b.parse::<i64>().unwrap();
            if i == 0 {
                (b, i)
            } else {
                //index, bus_id, remainder mod
                (b, b - (i % b))
            }
        })
        .collect();

    //Sorting to do the maximum first reduces our loops as we need to match smaller numbers.
    buses.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", buses);

    //minimum number can be the largest val
    let mut current = buses[0].1;
    let mut product = 1;

    //we ignore the last loop as we have it covered in our lookahead code below
    for i in 0..(buses.len() - 1) {
        //We can step each loop by this amount as it must match this whole number
        product *= buses[i].0;
        //we loop until we have a multiple with left over matching what we need
        while current % buses[i + 1].0 != buses[i + 1].1 {
            //increment next search by current step
            current += product;
        }
        println!("{:?} current - {:?} product - {:?}", i, current, product);
    }

    println!("Problem 2 - Lowest Time {:?}", current);
}

fn problem1(data: &Vec<String>) {
    let earliest = data[0].parse::<i32>().unwrap();
    let bus_ids: Vec<i32> = data[1]
        .split(',')
        .filter(|f| f != &"x")
        .map(|b| b.parse::<i32>().unwrap())
        .collect();

    let mut leave = earliest;
    let mut _bus_id = 0;

    loop {
        match bus_ids.iter().find(|a| leave % *a == 0) {
            Some(v) => {
                _bus_id = *v;
                break;
            }
            None => leave += 1,
        }
    }
    println!(
        "start:{:?} earliest:{:?} bus_id:{:?}",
        earliest, leave, _bus_id
    );
    println!("Problem 1. {:?}", (leave - earliest) * _bus_id);
}
