use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Coordinate {
    x: isize,
    y: isize
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Square {
    cost: isize,
    coordinate: Coordinate
}

impl Ord for Square {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut map: HashMap<Coordinate,isize> = HashMap::new();
    let mut max_x: isize = 0;
    let max_y = data.split('\n').count() as isize;
    for y_mul in 0..5
     {
        
        for (y, line) in data.split('\n').enumerate() {
            max_x = line.len() as isize;
            let y_offset = y_mul * max_y;
            for x_mul in 0..5 {
                for (x, c) in line.chars().enumerate(){
                    let x_offset = x_mul * max_x;
                    let value = ((String::from(c).parse::<isize>().unwrap() + x_mul + y_mul) -1) % 9 +1;
                    map.insert(Coordinate { x: x as isize + x_offset, y: y as isize + y_offset}, value);
                }
            }
        }      
    }
    println!("Part 1: {}", part1(&map, max_x, max_y));
    println!("Part 2: {}", part1(&map, max_x*5, max_y*5));
}


fn part1(map: &HashMap<Coordinate,isize>, max_x: isize, max_y: isize) -> isize {

    let mut costs: HashMap<Coordinate, isize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    let start = Coordinate{x:0,y:0};
    let goal = Coordinate{x: max_x-1, y: max_y -1};
    
    heap.push(Square{cost:0, coordinate: start});

    while !heap.is_empty() {
        let square = heap.pop().unwrap();
        if square.coordinate == goal{
            return square.cost;
        }
        if square.cost <= *costs.get(&square.coordinate).unwrap_or(&isize::MAX) { 
            for neighbour in get_neighbours(&square.coordinate) {
                if map.contains_key(&neighbour) {
                    let next_square = Square{cost: square.cost + map.get(&neighbour).unwrap(), coordinate: neighbour};
                    if next_square.cost < *costs.get(&neighbour).unwrap_or(&isize::MAX){
                        heap.push(next_square);
                        costs.insert(neighbour, next_square.cost);
                    }
                }
            }
        }

    }
    unreachable!()
}

fn get_neighbours(c: &Coordinate) -> Vec<Coordinate> {
    vec![Coordinate{x: c.x-1, y: c.y},Coordinate{x: c.x+1, y: c.y}, Coordinate{x: c.x, y: c.y+1}, Coordinate{x: c.x, y: c.y-1} ]
}