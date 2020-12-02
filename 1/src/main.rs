use std::fs::File;
use std::io::{BufReader,BufRead};


fn main() {
    let file_name  = "data.txt";
    let the_numbers = numbers(file_name); 

    let mut parts = Vec::<i32>::new();
    
    number_of_elements_equal( 2020, &the_numbers,2, &mut parts);
    println!("First Problem.......{:?} ,  {:?}  ", parts.iter().sum::<i32>(), parts.iter().product::<i32>());
    println!("First Problem Parts......{:?}", parts);


    parts.clear();

    number_of_elements_equal( 2020, &the_numbers,3, &mut parts);
    println!("Second Problem.......{:?} ,  {:?}  ", parts.iter().sum::<i32>(), parts.iter().product::<i32>());
    println!("Second Problem Parts......{:?}", parts);

}

fn numbers(file_name: &str) -> Vec<i32> {
    let file = File::open(file_name).unwrap();

    let reader = BufReader::new(file);
   
    return reader
        .lines()
        .map(| line | {
            line.unwrap().parse::<i32>().unwrap()
        })
        .collect();
}

fn find_number_two_values(value: i32, numbers: &Vec<i32>) -> (i32,i32) {
    for i in numbers{
        let left_over =  &(value - i); 
        if numbers.contains(left_over){
            return (*i, *left_over)
        }
    }
    return (0,0) 
}


fn number_of_elements_equal(value: i32, numbers: &Vec<i32>, elements: i32, parts: &mut Vec<i32> )
{
    //if there are two parts only required we need to find a number that equals the value 
    if elements == 2{
        let x = find_number_two_values(value, numbers);
        if x != (0,0) {
            parts.push(x.0);
            parts.push(x.1);
            return
        }
    }
        
    if elements > 2{
        for i in numbers{
            two_values_equal(value - i, numbers, elements - 1 , parts);
            if ! parts.is_empty() {
                parts.insert(0,*i);
                return
            }
        }
    }
}
