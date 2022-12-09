use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn full_path(path: &VecDeque<String>, extra: &str)-> String{
    let mut p = String::new();
    for dir in path{
        p += "/";
        p += dir;
    }
    p += "/";
    return p + extra
}

fn update_size(folders: &mut HashMap<String, u32>, curr_path: &VecDeque<String>, size: u32){
    let mut copied_path = curr_path.clone();
    loop{
        let fulldirname: String = full_path(&copied_path, "");
        if folders.contains_key(&fulldirname){
            *folders.get_mut(&fulldirname).unwrap() += size;
        } else {
            folders.insert(fulldirname.clone(), size);
        }
        if fulldirname == "/"{
            return
        }
        copied_path.pop_back();
    }
}

fn part1(input: &Vec<String>) -> u32 {
    let mut folders: HashMap<String, u32> = HashMap::new();
    let mut curr_path : VecDeque<String> = VecDeque::new();
    let mut already: Vec<String> = Vec::new();

    for line in input{
        let parts : Vec<&str> = line.split(" ").collect();
        if parts[0] == "$"{
            if parts[1] == "cd"{
                if parts[2] == ".." && curr_path.len() > 1{
                    curr_path.pop_back();
                } else {
                    curr_path.push_back(parts[2].to_owned());
                }
            } else if parts[1] == "ls"{

            }
        } else if parts[0] == "dir"{

        } else {
            let fullfilename: String = full_path(&curr_path, parts[1]);
            if ! already.contains(&fullfilename){
                already.push(fullfilename);  
                update_size(&mut folders, &curr_path, parts[0].parse::<u32>().unwrap());              
            }
        }
    }

    let mut sum = 0;
    for e in folders.keys(){
        if folders[e] <= 100000{
            sum += folders[e];
        }
    }
    return sum
}

fn part2(input: &Vec<String>) -> u32 {
    let mut folders: HashMap<String, u32> = HashMap::new();
    let mut curr_path : VecDeque<String> = VecDeque::new();
    let mut already: Vec<String> = Vec::new();

    for line in input{
        let parts : Vec<&str> = line.split(" ").collect();
        if parts[0] == "$"{
            if parts[1] == "cd"{
                if parts[2] == ".." && curr_path.len() > 1{
                    curr_path.pop_back();
                } else {
                    curr_path.push_back(parts[2].to_owned());
                }
            } else if parts[1] == "ls"{

            }
        } else if parts[0] == "dir"{

        } else {
            let fullfilename: String = full_path(&curr_path, parts[1]);
            if ! already.contains(&fullfilename){
                already.push(fullfilename);  
                update_size(&mut folders, &curr_path, parts[0].parse::<u32>().unwrap());              
            }
        }
    }

    let need2free = 30000000 - (70000000 - folders["/"]); // needed space minus free space


    let mut min = u32::MAX;
    for e in folders.keys(){
        if folders[e] >= need2free && folders[e] <= min{
            min = folders[e];
        }
    }
    return min
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
