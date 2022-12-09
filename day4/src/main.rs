use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn part1(input: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in input{
        let quad : Vec<i32> = line.split(&['-',','][..]).map(|x| x.parse::<i32>().unwrap()).collect();

        if quad[0] >= quad[2] && quad[1] <= quad[3] || quad[0] <= quad[2] && quad[1] >= quad[3]{
            sum += 1;
        } 
    }
    return sum
}

fn part2(input: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in input{
        let quad : Vec<i32> = line.split(&['-',','][..]).map(|x| x.parse::<i32>().unwrap()).collect();

        if quad[1] >= quad[2] && quad[0] <= quad[3]{
            sum += 1;
        } 
    }
    return sum
}

fn main(){
    let file = File::open("input.txt").expect("cant open!");
    let input = BufReader::new(file);

    let mut pro_input = Vec::new();
    for line in input.lines(){
        pro_input.push(line.expect("no line"));
    }

    println!("part 1: {}", part1( &pro_input ) );
    println!("part 2: {}", part2( &pro_input ) );
}
