use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// let x : Vec<_> = Vec::new();
//line.split(" ").collect();
// a rock b paper c scissors X Y Z  1 2 3 for selected . plus 0 3 or 6 win draw loss

fn part1(input: &Vec<String>) -> i32 {
    let mut total = 0;
    for line in input{
        let mut score = 0;
        let choices : Vec<&str> = line.split(" ").collect();
        if choices[0] == "A"{
            if choices [1] == "X"{
                score += 1;
                score += 3;
            }
            else if choices[1] == "Y"{
                score += 2;
                score += 6;
            }
            else if choices[1] == "Z"{
                score += 3;
                score += 0;
            }
        }
        else if choices[0] == "B"{
            if choices [1] == "X"{
                score += 1;
                score += 0;
            }
            else if choices[1] == "Y"{
                score += 2;
                score += 3;
            }
            else if choices[1] == "Z"{
                score += 3;
                score += 6;
            }
        }
        else if choices[0] == "C"{
            if choices [1] == "X"{
                score += 1;
                score += 6;
            }
            else if choices[1] == "Y"{
                score += 2;
                score += 0;
            }
            else if choices[1] == "Z"{
                score += 3;
                score += 3;
            }
        }
        total += score;
    }
    return total
}

fn part2(input: &Vec<String>) -> i32 {
    let mut total = 0;
    for line in input{
        let mut score = 0;
        let choices : Vec<&str> = line.split(" ").collect();
        if choices[0] == "A"{
            if choices [1] == "X"{
                score += 3;
                score += 0;
            }
            else if choices[1] == "Y"{
                score += 1;
                score += 3;
            }
            else if choices[1] == "Z"{
                score += 2;
                score += 6;
            }
        }
        else if choices[0] == "B"{
            if choices [1] == "X"{
                score += 1;
                score += 0;
            }
            else if choices[1] == "Y"{
                score += 2;
                score += 3;
            }
            else if choices[1] == "Z"{
                score += 3;
                score += 6;
            }
        }
        else if choices[0] == "C"{
            if choices [1] == "X"{
                score += 2;
                score += 0;
            }
            else if choices[1] == "Y"{
                score += 3;
                score += 3;
            }
            else if choices[1] == "Z"{
                score += 1;
                score += 6;
            }
        }
        total += score;
    }
    return total
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
