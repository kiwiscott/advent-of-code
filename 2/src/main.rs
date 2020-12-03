use std::fs::File;
use std::io::{BufReader,BufRead};
use std::collections::HashMap;


fn main() {
    let file_name  = "data.txt";
    let passwords = passwords(file_name); 

    let r = passwords.iter()
    .map(|p| valid_password_policy_1(p))
    .filter(|x| *x)
    .count();
    
    println!("First Problem Parts......{:?}", r);

    let rr = passwords.iter()
        .map(|p| valid_password_policy_2(p))
        .filter(|x| *x)
        .count();

    println!("Second Problem Parts......{:?}", rr);
    

}

//8-9 f: fffffffxx
#[derive(Debug)]
struct Password {
    min: i32,
    max: i32,
    letter: char,
    password: String
}

fn valid_password_policy_2(p: &Password) -> bool
{
    let char_vec: Vec<char> = p.password.chars().collect();
   
    let m = (p.min - 1) as usize; 
    let mm = (p.max - 1) as usize; 

    
    
    //Both cannot be true
    if (char_vec[m] == p.letter && char_vec[mm] == p.letter) {
        return false; 
    }
    return  char_vec[m] == p.letter || char_vec[mm] == p.letter;
}

fn valid_password_policy_1(p: &Password) -> bool
{
    let mut char_map: HashMap<char, i32> = HashMap::new();


    for c in p.password.chars(){
        if char_map.contains_key(&c)
        {
            let i = 1 + char_map[&c]; 
            char_map.remove(&c);
            char_map.insert(c, i);
        }
        else{
            char_map.insert(c, 1);
        }
    }
    

    return char_map.contains_key(&p.letter) && (p.min <= char_map[&p.letter]) &&  char_map[&p.letter] <= p.max

    

}
fn passwords(file_name: &str) -> Vec<Password> {
    let mut pwds: Vec<Password> = Vec::new();

    let file = File::open(file_name).unwrap();

    let reader = BufReader::new(file);


    for line in reader.lines(){
        let as_string = line.unwrap();

        let mut elements = as_string.split(|v| v == ' ' || v == '-' || v == ':' );

        //let pp: Vec<&str> =as_string.split(|v| v == ' ' || v == '-' || v == ':' ).collect();
        //println!("First Problem Parts......{:?}", pp);


        let p = Password {
            min: elements.next().unwrap().parse::<i32>().unwrap(),
            max: elements.next().unwrap().parse::<i32>().unwrap(),
            letter: elements.next().unwrap().parse::<char>().unwrap(),
            password: elements.last().unwrap().to_string(),
        };

        pwds.push(p);

    }

    return pwds;

}