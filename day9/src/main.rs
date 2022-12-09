use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn follow_head( head: (i32, i32), tail: (i32, i32)) -> (i32,i32){
    if head.0 == tail.0{
        if (head.1-tail.1).abs() <= 1{
            return tail
        }
        else if head.1 > tail.1{
            return (tail.0, tail.1 + 1)
        }
        else{// if head.1 < tail.1{
            return (tail.0, tail.1 - 1);
        }
    } else if head.1 == tail.1{
        if (head.0-tail.0).abs() <= 1{
            return tail
        }
        else if head.0 > tail.0{
            return (tail.0 + 1, tail.1);
        }
        else{// if head.0 < tail.0{
            return (tail.0 - 1, tail.1);
        }
    } else { // 0 and 1 are different
        if ( (head.0-tail.0).abs() + (head.1-tail.1).abs() ) == 2{
            return tail
        } else{
            if head.0 > tail.0{
                if head.1 > tail.1{
                    return (tail.0 +1, tail.1 +1);
                }else {
                    return (tail.0 +1, tail.1 -1);
                }
            } else{
                if head.1 > tail.1{
                    return (tail.0 -1, tail.1 +1);
                }else {
                    return (tail.0 -1, tail.1 -1);
                } 
            }
        }
    }
}

fn part1(input: &Vec<String>) -> u32 {

    let mut grid : HashMap<(i32,i32), bool> = HashMap::new();
    let mut h : (i32,i32) = (0, 0);
    let mut t : (i32,i32) = (0, 0);
    
    for line in input{
        let instruction : Vec<&str> = line.split(" ").collect();
        for _step in 0..instruction[1].parse::<usize>().unwrap(){
            match instruction[0]{
                "R" => { 
                    if t.1 < h.1{
                        t.1 += 1;
                        t.0 = h.0;
                    }
                    h.1 += 1;
                },
                "L" => {
                    if t.1 > h.1{
                        t.1 -= 1;
                        t.0 = h.0
                    }
                    h.1 -= 1;
                },
                "U" => {
                    if t.0 < h.0{
                        t.0 += 1;
                        t.1 = h.1;
                    }
                    h.0 += 1;
                },
                _ => { // "D"
                    if t.0 > h.0{
                        t.0 -= 1;
                        t.1 = h.1;
                    }
                    h.0 -= 1;
                }            
            };
            grid.insert(t, true);
        }
    }

    return grid.keys().count() as u32;
}

fn part2(input: &Vec<String>) -> u32 {
    let mut grid : HashMap<(i32,i32), bool> = HashMap::new();
    let mut poss : HashMap<usize,(i32,i32)> = HashMap::new();
    
    for knot in 0..10{
        poss.insert(knot, (0,0));
    }
    
    for line in input{
        let instruction : Vec<&str> = line.split(" ").collect();
        for _step in 0..instruction[1].parse::<usize>().unwrap(){    
            //move the head        
            poss.insert(0, match instruction[0]{
                "R" => (poss[&0].0, (poss[&0].1)+1),
                "L" => (poss[&0].0, (poss[&0].1)-1),
                "U" => ((poss[&0].0)+1, poss[&0].1),
                _ => ((poss[&0].0)-1, poss[&0].1)
            });
            //follow
            for knot in 1..10{
                poss.insert(knot,follow_head(poss[&(knot-1)], poss[&knot]));
            }
            grid.insert(poss[&9], true);
        }
    }

    return grid.keys().count() as u32;
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
