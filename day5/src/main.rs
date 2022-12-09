use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::VecDeque;

        //let quad : Vec<i32> = line.split(&['-',','][..]).map(|x| x.parse::<i32>().unwrap()).collect();

fn part1(input: &Vec<String>) -> String {
    let mut done_stacking = false;
    let mut all : Vec<VecDeque<char>> = Vec::new();
    for _i in 0..9{
        all.push(VecDeque::new());
    }
    for line in input{
        if done_stacking == false{
            if line == " 1   2   3   4   5   6   7   8   9 "{
                done_stacking = true;
            } else {
                let linechars : Vec<char> = line.chars().collect();
                for index in 0..9{
                    let element = linechars[index*4 + 1];
                    if element != ' '{
                        all[index].push_back(element);
                    }
                }
            }
        } else if line.len() < 3{
            println!("{:?}", all);
        } else {
            let instruction : Vec<usize> = line.replace("move ", "")
                                                  .replace(" from ", "f")
                                                  .replace(" to ", "t")
                                                  .split(&['f','t'][..])
                                                  .map(|x| x.parse::<usize>().unwrap())
                                                  .collect();
            for _i in 0..instruction[0]{
                let one_crate = all[instruction[1]-1].pop_front().unwrap();
                all[instruction[2]-1].push_front(one_crate);
            }
        }
    }
    
    let mut output = String::new();
    for c in all.iter_mut(){
        output.push(c.pop_front().unwrap());
    }

    return output
}

fn part2(input: &Vec<String>) -> String {
    let mut done_stacking = false;
    let mut all : Vec<VecDeque<char>> = Vec::new();
    for _i in 0..9{
        all.push(VecDeque::new());
    }
    for line in input{
        if done_stacking == false{
            if line == " 1   2   3   4   5   6   7   8   9 "{
                done_stacking = true;
            } else {
                let linechars : Vec<char> = line.chars().collect();
                for index in 0..9{
                    let element = linechars[index*4 + 1];
                    if element != ' '{
                        all[index].push_back(element);
                    }
                }
            }
        } else if line.len() < 3{
            println!("{:?}", all);
        } else {
            let instruction : Vec<usize> = line.replace("move ", "")
                                                  .replace(" from ", "f")
                                                  .replace(" to ", "t")
                                                  .split(&['f','t'][..])
                                                  .map(|x| x.parse::<usize>().unwrap())
                                                  .collect();
            
            let mut crane = VecDeque::new();
            
            for _i in 0..instruction[0]{
                let one_crate = all[instruction[1]-1].pop_front().unwrap();
                crane.push_front(one_crate);
            }
            for e in 0..crane.len(){
                all[instruction[2]-1].push_front(crane.pop_front().unwrap());
            }

        }
    }
    
    let mut output = String::new();
    for c in all.iter_mut(){
        output.push(c.pop_front().unwrap());
    }

    return output
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
