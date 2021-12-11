use std::collections::HashMap;

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Coordinate {
    x: isize,
    y: isize
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Octopus {
        energy: i32
}
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut map: HashMap<Coordinate,Octopus> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.split('\n').enumerate() {
        max_y +=1;
        max_x = line.len() as isize;
        for (x, c) in line.chars().enumerate(){
            map.insert(Coordinate { x: x as isize, y: y as isize}, Octopus{energy: String::from(c).parse::<i32>().unwrap()});
        }
    }
    let mut steps = 0;
    let mut found = false;
    let mut flash_count = 0;
    let locations = &map.keys().copied().collect::<Vec<_>>();
    while ! found {        
        steps += 1;
        for loc in locations {
            let mut octopus = map.get_mut(loc).unwrap();
            octopus.energy += 1;
        }
        let mut flashed = true ; 
        while flashed {
            flashed = false;
            for loc in locations {
                let mut octopus = map.get_mut(loc).unwrap();
                if octopus.energy > 9 {
                    flash_count += 1;
                    octopus.energy = 0;
                    flashed = true;
                    for n in get_neighbours(loc) {
                        if map.contains_key(&n) {
                            let neighbour =  map.get_mut(&n).unwrap();
                            if neighbour.energy != 0 {
                                neighbour.energy +=1;
                            }
                        }
                    }    
                }
            }
        }
        if steps == 100 {
            println!("Part 1: Flash count: {}", flash_count);
        }
        if map.values().filter(|x| x.energy == 0).count() as isize == (max_x * max_y) {
            println!("Part 2: Step: {}", steps);
            found = true;
        }
    }
}


fn get_neighbours(c: &Coordinate) -> Vec<Coordinate> {
    vec![Coordinate{x: c.x-1, y: c.y},Coordinate{x: c.x+1, y: c.y}, Coordinate{x: c.x, y: c.y+1}, Coordinate{x: c.x, y: c.y-1},
        Coordinate{x: c.x-1,y: c.y-1}, Coordinate{x: c.x+1, y: c.y+1},  Coordinate{x: c.x-1,y: c.y+1}, Coordinate{x: c.x+1, y: c.y-1}]
}


fn print_grid(max_x: isize, max_y: isize, floor_map: &HashMap<Coordinate, Octopus>) {
    for y in 0..max_y {
        for x in 0..max_x {
            print!("{}", floor_map.get(&Coordinate{x,y}).unwrap().energy);
        }
        println!();
    }
}