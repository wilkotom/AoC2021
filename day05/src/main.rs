use std::collections::HashMap;

#[derive(Debug,Hash,Eq,PartialEq,Copy,Clone)]
struct Coordinate {
    x: isize,
    y: isize
}
fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut part1_map: HashMap<Coordinate,isize> = HashMap::new();
    let mut part2_map: HashMap<Coordinate,isize> = HashMap::new();
    for line in data.split("\n"){
        let points = line.split(" -> ").collect::<Vec<_>>();
        let start = to_coordinate(&points[0]);
        let end = to_coordinate(&points[1]);        
        if start.x == end.x {
            for y in std::cmp::min(start.y, end.y)..std::cmp::max(start.y, end.y) + 1 {
                part1_map.insert(Coordinate{x: start.x,y}, part1_map.get(&Coordinate{x: start.x,y}).unwrap_or(&0) + 1);
                part2_map.insert(Coordinate{x: start.x,y}, part2_map.get(&Coordinate{x: start.x,y}).unwrap_or(&0) + 1);
            }
        } else { 
            let x_step = if start.x < end.x {1} else {-1};
            let y_step = if start.y < end.y {1} else if start.y > end.y {-1} else {0};
            let mut x = start.x;
            let mut y = start.y;
            while (x <= end.x && x_step > 0) ||  (x >= end.x && x_step < 0) {
                if y_step == 0 { 
                    part1_map.insert(Coordinate{x,y}, part1_map.get(&Coordinate{x,y}).unwrap_or(&0) + 1);
                }
                part2_map.insert(Coordinate{x,y}, part2_map.get(&Coordinate{x,y}).unwrap_or(&0) + 1);
                x += x_step;
                y += y_step;
            }
        }
    }
    println!("Part 1: {}", part1_map.values().filter(|n| **n >= 2).count());
    println!("Part 2: {}", part2_map.values().filter(|n| **n >= 2).count());
}

fn to_coordinate(coords: &str) -> Coordinate {
    let axes = coords.split(",").collect::<Vec<_>>();
    let x = axes[0].parse::<isize>().unwrap();
    let y = axes[1].parse::<isize>().unwrap();
    Coordinate{x,y}
}