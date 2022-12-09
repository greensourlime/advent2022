use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn all_unique(quad: &VecDeque<char>) -> bool{
    for i in 0..quad.len(){
        for j in i+1..quad.len(){
            if quad[i] == quad[j]{
                return false
            }
        }
    }
    return true
}

fn part1(input: &Vec<String>) -> i32 {
    for line in input{
        let mut linechars = line.chars();
        let mut four : VecDeque<char> = VecDeque::new();
        let block_definition = 14;

        for _i in 0..block_definition{
            four.push_back(linechars.next().unwrap());
        }
        let mut index = block_definition;
        while !all_unique(&four){
            four.pop_front();
            four.push_back(linechars.next().unwrap());
            index += 1;
        }
        return index
    }
    return -1
}

fn main(){
    let file = File::open("input.txt").expect("cant open!");
    let input = BufReader::new(file);

    let mut pro_input = Vec::new();
    for line in input.lines(){
        pro_input.push(line.expect("no line"));
    }

    println!("part 1: {}", part1( &pro_input ) );
    //println!("part 2: {}", part2( &pro_input ) );
}
