use std::collections::{HashMap, VecDeque};

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let corrupt_score = HashMap::from([(')',3),(']', 57),('}', 1197), ('>', 25137)]);
    let incomplete_score = HashMap::from([('(',1),('[', 2), ('{', 3), ('<', 4)]);
    let opening = HashMap::from([(']','['), ('}','{'), ('>','<'), (')','(')]);
    let mut part1_score = 0;
    let mut part2_scores: Vec<isize> = Vec::new();
    for line in input.split('\n'){
        let mut stack : VecDeque<char> = VecDeque::new();
        let mut complete = true;
        let mut line_score = 0;
        for c in line.chars() {
            match c {
                '{' | '(' | '[' | '<' => {
                    stack.push_back(c)
                }
                _ => {
                    let last = stack.pop_back().unwrap();
                    if last != *opening.get(&c).unwrap() {
                        part1_score += corrupt_score.get(&c).unwrap();
                        complete = false;
                        break;
                    }
                }
            }
        }
        if complete {
            while stack.len() > 0 {
                let c = stack.pop_back().unwrap();
                line_score = (line_score * 5) + incomplete_score.get(&c).unwrap();
            }
            part2_scores.push(line_score);
        }
    }
    println!("Part 1 Score: {}", part1_score);
    part2_scores.sort();
    println!("Part 2 Winning Score: {}", part2_scores[part2_scores.len() / 2]);
}

