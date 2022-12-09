use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// let x : Vec<_> = Vec::new();
//line.split(" ").collect();
// a rock b paper c scissors X Y Z  1 2 3 for selected . plus 0 3 or 6 win draw loss

fn priority(c: u32)->u32{
    if c > 96{ // lowercase
        return c-96
    } else {
        return c-65+27
    }
}

fn part1(input: &Vec<String>) -> u32 {
    let mut sum : u32 = 0;
    for line_s in input{
        let line : Vec<char> = line_s.chars().collect();
        let cut = line.len()/2;
        
        'outer: for i in 0..cut{
            for j in cut..line.len(){
                if line[i] == line[j]{
                    let code = line[i] as u32;                
                    sum+= priority(code);
                    //println!("Added {} which is {} for a priority of {}", line[i], code, priority(code));
                    break 'outer    
                }
            }
        }
    }
    return sum
}

fn part2(input: &Vec<String>) -> u32 {
    let mut sum : u32 = 0;

    for i in 0..input.len()/3{
        'outer: for e_one in input[i*3].chars(){
            for e_two in input[i*3+1].chars(){
                if e_one == e_two{
                    for e_three in input[i*3+2].chars(){
                        if e_one == e_three{
                            sum += priority(e_one as u32);
                            break 'outer
                        }
                    }
                }
            }
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

    //println!("part 1: {}", part1( &pro_input ) );
    println!("part 2: {}", part2( &pro_input ) );
}
