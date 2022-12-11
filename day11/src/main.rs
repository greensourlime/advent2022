use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Clone,Debug)]
enum Operation{
    Adding,
    //Substracting,
    Multiplying,
    Dividing,
    Pow
}
#[derive(Clone,Debug)]
struct Monkey{
    items: VecDeque<u128>,
    operation: Operation,
    op_param: u128,
    divisible: u128,
    if_true: usize,
    if_false: usize
}

impl Default for Monkey{
    fn default () -> Monkey{
        Monkey { items: VecDeque::new(), operation: Operation::Adding, op_param: 0, divisible: 1, if_true: 1, if_false: 1 }
    }
}


fn part1(input: &Vec<String>) -> i64 {
    
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();    
    let mut monkey: Monkey = Monkey::default();
    let mut monkey_number : usize = 0;

    for line in input{
        if line.len() > 0{
            let params : Vec<&str> = line.trim().split(" ").collect();
            if params[0] == "Monkey"{
                monkey_number = params[1].replace(":","").parse::<usize>().unwrap();
                monkey = Monkey::default();                
            } else if params[0] == "Starting" {
                for p in 2..params.len(){
                    monkey.items.push_back(params[p].replace(",", "").parse::<u128>().unwrap());
                }
            } else if params[0] == "Operation:" {
                monkey.operation = match params[4]{
                    "+" => Operation::Adding,
                    //"-" => Operation::Substracting,
                    "*" => Operation::Multiplying,
                    _ => Operation::Dividing
                };
                if params[5] == "old"{
                    monkey.operation = Operation::Pow;
                } else{
                    monkey.op_param = params[5].parse::<u128>().unwrap();
                }
            } else if params[0] == "Test:"{
                monkey.divisible = params[3].parse::<u128>().unwrap();
            } else if params[0] == "If"{
                if params[1] == "true:"{
                    monkey.if_true = params[5].parse::<usize>().unwrap();
                } else {
                    monkey.if_false = params[5].parse::<usize>().unwrap();                    
                }
            }
        } else{
            monkeys.insert(monkey_number, monkey.clone());
        }
    }
    monkeys.insert(monkey_number, monkey);

    let mut inspections : HashMap<usize, usize> = HashMap::new();
    for monkey in 0..monkeys.keys().count(){
        inspections.insert(monkey, 0);
    }

    for _round in 0..20{
        for monkey in 0..monkeys.keys().count(){
            let mut throw_stacks = Vec::new();
            let m = monkeys.get_mut(&monkey).unwrap();
            while let Some(item) = m.items.pop_front(){
                *inspections.get_mut(&monkey).unwrap() += 1;
                let mut item_new = match m.operation{
                    Operation::Adding => item + m.op_param,
                    //Operation::Substracting => item - m.op_param,
                    Operation::Multiplying => item * m.op_param,
                    Operation::Pow => item * item, 
                    _ => item / m.op_param
                };
                item_new = (item_new as f64 / 3.0).floor() as u128;
                if item_new % m.divisible == 0{
                    throw_stacks.push((item_new, m.if_true));
                } else {
                    throw_stacks.push((item_new, m.if_false));                    
                }
            }
            for throw in throw_stacks{
                monkeys.get_mut(&throw.1).unwrap().items.push_back(throw.0);
            }
        }
    }    

    let mut scores = Vec::new();
    for m in inspections.keys(){
        scores.push(inspections[&m]);
    }
    scores.sort();
    
    return (scores[scores.len()-1]*scores[scores.len()-2]) as i64

}


fn part2(input: &Vec<String>) -> i64 {
    
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();    
    let mut monkey: Monkey = Monkey::default();
    let mut monkey_number : usize = 0;
    let mut min_common_mult = 1;

    for line in input{
        if line.len() > 0{
            let params : Vec<&str> = line.trim().split(" ").collect();
            if params[0] == "Monkey"{
                monkey_number = params[1].replace(":","").parse::<usize>().unwrap();
                monkey = Monkey::default();                
            } else if params[0] == "Starting" {
                for p in 2..params.len(){
                    monkey.items.push_back(params[p].replace(",", "").parse::<u128>().unwrap());
                }
            } else if params[0] == "Operation:" {
                monkey.operation = match params[4]{
                    "+" => Operation::Adding,
                    //"-" => Operation::Substracting,
                    "*" => Operation::Multiplying,
                    _ => Operation::Dividing
                };
                if params[5] == "old"{
                    monkey.operation = Operation::Pow;
                } else{
                    monkey.op_param = params[5].parse::<u128>().unwrap();
                }
            } else if params[0] == "Test:"{
                monkey.divisible = params[3].parse::<u128>().unwrap();
                min_common_mult *= monkey.divisible;
            } else if params[0] == "If"{
                if params[1] == "true:"{
                    monkey.if_true = params[5].parse::<usize>().unwrap();
                } else {
                    monkey.if_false = params[5].parse::<usize>().unwrap();                    
                }
            }
        } else{
            monkeys.insert(monkey_number, monkey.clone());
        }
    }
    monkeys.insert(monkey_number, monkey);

    let mut inspections : HashMap<usize, usize> = HashMap::new();
    for monkey in 0..monkeys.keys().count(){
        inspections.insert(monkey, 0);
    }

    for _round in 0..10000{
        for monkey in 0..monkeys.keys().count(){
            let mut throw_stacks = Vec::new();
            let m = monkeys.get_mut(&monkey).unwrap();
            while let Some(item) = m.items.pop_front(){
                *inspections.get_mut(&monkey).unwrap() += 1;
                let mut item_new = match m.operation{
                    Operation::Adding => item + m.op_param,
                    //Operation::Substracting => item - m.op_param,
                    Operation::Multiplying => item * m.op_param,
                    Operation::Pow => item * item, 
                    _ => item / m.op_param                };
                //item_new = (item_new as f64 / 3.0).floor() as i64;
                item_new = item_new % min_common_mult;
                if item_new % m.divisible == 0{
                    throw_stacks.push((item_new, m.if_true));
                } else {
                    throw_stacks.push((item_new, m.if_false));                    
                }
            }
            for throw in throw_stacks{
                monkeys.get_mut(&throw.1).unwrap().items.push_back(throw.0);
            }
        }
    }    

    let mut scores : Vec<usize> = Vec::new();
    for m in inspections.keys(){
        scores.push(inspections[&m]);
    }
    scores.sort();
    
    return (scores[scores.len()-1]*scores[scores.len()-2]) as i64

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
