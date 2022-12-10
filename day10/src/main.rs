use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn part1(input: &Vec<String>) -> i32 {

    let mut reg_x = 1;
    let checkpoints = vec![20,60,100,140,180,220];
    let mut signal :Vec<i32> = Vec::new();
    let mut cycles = 0;
    for line in input{
        let params : Vec<&str> = line.split(" ").collect();
        cycles += 1;
        if checkpoints.contains(&cycles){
            signal.push(reg_x * cycles);
        }
        if params[0] == "addx"{
            cycles += 1;

            if checkpoints.contains(&cycles){
                signal.push(reg_x * cycles);
            }

            reg_x += params[1].parse::<i32>().unwrap();
        }
    }
    
    return signal.iter().sum()
}

fn part2(input: &Vec<String>) -> i32 {
    let mut reg_x: i32 = 1;
    let mut cycles = 0;
    let mut crt : Vec<String> = Vec::new();
    let mut crt_row: String = String::new();
    crt_row.push('\u{2593}');

    for line in input{
        let params : Vec<&str> = line.split(" ").collect();

        cycles += 1;

        if cycles%40==0 && cycles/40 >0{
            crt.push(crt_row);
            crt_row = String::new();
        }
        crt_row.push(if (reg_x-cycles%40).abs() > 1 {' '} else {'\u{2593}'});
            
        if params[0] == "addx"{
            cycles += 1;

            reg_x += params[1].parse::<i32>().unwrap();

            if cycles%40==0 && cycles/40 >0{
                crt.push(crt_row);
                crt_row = String::new();
            }
            crt_row.push(if (reg_x-cycles%40).abs() > 1 {' '} else {'\u{2593}'});
        }
    }
    println!("");
    for row in crt{
        println!("{}", row);
    }
    println!("");

    return 0
}


fn main(){
    let file = File::open("input.txt").expect("cant open!");
    let input = BufReader::new(file);

    let mut pro_input = Vec::new();
    for line in input.lines(){
        pro_input.push(line.expect("no line"));
    }

    println!("part 1: {}", part1( &pro_input ) );
    println!("part 2: ");
    part2( &pro_input );
}
