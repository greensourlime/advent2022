use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn part1(input: &Vec<String>) -> i32 {
    let mut max = 0;
    let mut current_elf: i32 = 0;
    for line in input{
        if line.len() < 1{
            if current_elf > max{
                max = current_elf;
            }
            current_elf = 0;
        }
        else{
            current_elf += line.parse::<i32>().unwrap();
        }
    }
    return max
}

fn part2(input: &Vec<String>) -> i32 {
    let mut all : Vec<i32> = Vec::new();
    let mut current_elf: i32 = 0;
    for line in input{
        if line.len() < 1{
            all.push(current_elf);
            current_elf = 0;
        }
        else{
            current_elf += line.parse::<i32>().unwrap();
        }
    }
    //all.sort_by(|a, b| a.partial_cmp(b).unwrap());
    all.sort();
    let answer = all[all.len()-1] + all[all.len()-2] + all[all.len()-3];
    return answer
} 
fn main(){
    let file = File::open("input.txt").expect("cant open!");
    let input = BufReader::new(file);

    let mut pro_input = Vec::new();
    for line in input.lines(){
        pro_input.push(line.expect("no line"));
    }

    //println!("part 1: {}", part1( &pro_input ) );
    println!("part 2: {}", part2( &pro_input ) );
}
