use aoc::common::*;
use std::collections::VecDeque; 
use std::cmp::Ordering; 

fn main() {
    let preamble_len = 25; 
    let numbers: Vec<i64>= read_file("input.txt").unwrap()
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    let p1 = match  problem1(preamble_len, numbers.clone()){
        Some(n) => {
            println!("Problem 1.Number [{:?}] is not a sum of its preamble numbers",n);
            n
        },
        None => {
                println!("Problem 1. All numbers are valid");
                i64::MIN
    }};

    if p1 == i64::MIN{
       return; 
    }


    match problem2(numbers.clone(),p1){
        Some(n) => println!("Problem 2: Sum {:?} [{:?}]",n.0+n.1,n), 
        None => println!("Problem 2: All numbers are valid"), 
    }
    println!("Should BE: 1261309 [(408514, 852795)]");

}
fn problem2(numbers: Vec<i64>, number_to_find: i64) -> Option<(i64,i64)>
{
    let mut nums_to_sum: VecDeque<i64> = VecDeque::new();
    let mut result = (0 as i64, 0 as i64);

    let mut index = 0; 
    let mut sum = 0;
    while index < numbers.len(){
        match sum.cmp(&number_to_find) {
            Ordering::Less => {
                    nums_to_sum.push_back(numbers[index]);
                    sum = sum + numbers[index];
                    index = index+1;
            },
            Ordering::Greater => { 
                sum = sum - nums_to_sum.pop_front().unwrap(); 
            },
            Ordering::Equal => {
                let min = nums_to_sum.iter().map(|n|*n).min().unwrap();
                let max = nums_to_sum.iter().map(|n|*n).max().unwrap();
                result = (min,max);
                println!("Result:{:?} s:{:?} nums:{:?}", number_to_find, sum, nums_to_sum );
                break;
            }
        }
    }

    if result ==  (0 as i64, 0 as i64)
    {
        return None; 
    }

    Some(result)

}



fn problem1(preamble_length: usize,numbers: Vec<i64> ) -> Option<i64>{
    //What is the first number that does cannot be represented by a sum of two numbers in the previous 'preamble' numbers? 
    //Build to preamble 
    let mut preamble: VecDeque<i64> = VecDeque::with_capacity(preamble_length);
    numbers.iter().take(preamble_length).for_each(|n| preamble.push_back(*n));

    //println!("Preamble {:?}-{:?}",preamble.len(),preamble);

    
    for n in numbers.iter().skip(preamble_length){
        let found = preamble.iter().any(|a|{
            preamble.contains(&(n-a))
        });
        if !found{
            //println!("Looking for n:[{:?}] [{:?}] in {:?}", n,found ,preamble); 
            return Some(*n);
        }
    
        //push this to the back of our queue 
        preamble.pop_front();   
        preamble.push_back(*n);               
    }
    return None; 
}
