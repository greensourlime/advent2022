use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn scene_score(forest: &HashMap<(u32,u32), u32>, tree: (u32,u32), dims: (u32,u32)) -> u32{
    let height = forest[&tree];
    // up
    let mut trees = 0;
    for r in (0..tree.0).rev(){
        trees += 1;
        if forest[&(r,tree.1)] >= height{
            break
        }
    }
    let mut score = trees;
    // down
    trees = 0;
    for r in tree.0+1..dims.1{
        trees += 1;
        if forest[&(r,tree.1)] >= height{
            break
        }
    }
    score *= trees;
    // left
    trees = 0;
    for c in (0..tree.1).rev(){
        trees += 1;
        if forest[&(tree.0,c)] >= height{
            break
        }
    }
    score *= trees;

    // right
    trees = 0;
    for c in tree.1+1..dims.1{
        trees += 1;
        if forest[&(tree.0,c)] >= height{
            break
        }
    }

    return score*trees
}

fn is_visible(forest: &HashMap<(u32,u32), u32>, tree: (u32,u32), dims: (u32,u32)) -> u32{
    if tree.0 == dims.0 || tree.0 == 0 || tree.1 == 0 || tree.1 == dims.1{
        return 1
    }

    let height = forest[&tree];
    let mut visible = true;
    // up
    for r in 0..tree.0{
        if forest[&(r, tree.1)] >= height{
            visible = false;
            break;
        }
    }
    if visible{
        return 1
    }
    visible = true;
    // down
    for r in tree.0+1..dims.0{
        if forest[&(r, tree.1)] >= height{
            visible = false;
            break
        }
    }
    if visible{
        return 1
    }
    visible = true;
    // left
    for c in 0..tree.1{
        if forest[&(tree.0, c)] >= height{
            visible = false;
            break
        }
    }
    if visible{
        return 1
    }
    visible = true;

    // right
    for c in tree.1+1..dims.1{
        if forest[&(tree.0, c)] >= height{
            visible = false;
            break
        }
    }
    if visible{
        return 1
    }
    return 0
    
}

fn part1(input: &Vec<String>) -> u32 {    
    let mut rows = 0;
    let mut cols = 0;
    let mut forest: HashMap<(u32,u32),u32> = HashMap::new();
    for line in input{
        cols = line.len();
        for (column, height) in line.chars().enumerate(){
            forest.insert((rows, column as u32), height as u32);
        }
        rows += 1;        
    }

    let mut visible = 0;
    for r in 0..rows{
        for c in 0..cols{
            visible += is_visible(&forest, (r,c as u32), (rows as u32,cols as u32));
        } 
    }
    return visible
}


fn part2(input: &Vec<String>) -> u32 {    
    let mut rows = 0;
    let mut cols = 0;
    let mut forest: HashMap<(u32,u32),u32> = HashMap::new();
    for line in input{
        cols = line.len();
        for (column, height) in line.chars().enumerate(){
            forest.insert((rows, column as u32), height as u32);
        }
        rows += 1;        
    }

    let mut best_scene = 0;
    for r in 0..rows{
        for c in 0..cols{
            let score = scene_score(&forest, (r,c as u32), (rows as u32,cols as u32));
            if score > best_scene{
                best_scene = score;
            }
        } 
    }
    return best_scene
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
