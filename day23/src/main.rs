use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

use hashbrown::HashSet;

// use hashbrown::HashMap;



#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize
}

impl Coordinate {
    fn get_neighbours(&self) -> Vec<Coordinate> {
        let x = self.x;
        let y = self.y;
        vec![Coordinate{x: x+1, y}, 
             Coordinate{x: x-1 , y},
             Coordinate{x, y:  y+ 1},
             Coordinate{x, y:  y+ -1}]
    }
}
#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze, 
    Copper,
    Desert
}

#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]
enum Square {
    Empty,
    Occupied(Amphipod),
    Forbidden
}

#[derive(Clone,Debug, PartialEq, Eq, Hash)]
struct GameState {
    cost: isize,
    waiting_spots: Vec<Square>,
    bins: Vec<Vec<Amphipod>>
}

#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]

enum BinType {
    Blocked,
    Amphipods(Amphipod)
}



impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() {
    let data = std::fs::read_to_string("./test.txt").unwrap();
    let mut lines = data.split('\n');
    let _ = lines.next();
    let corridor = lines.next().unwrap();
    let mut waiting_spots = Vec::new();
    for (i, c) in corridor.chars().enumerate() {
        waiting_spots.push(
            match c {
                '#' => Square::Forbidden,
                '.' => Square::Empty,
                'A' => Square::Occupied(Amphipod::Amber),
                'B' => Square::Occupied(Amphipod::Bronze),
                'C' => Square::Occupied(Amphipod::Copper),
                'D' => Square::Occupied(Amphipod::Desert),
                _ => unreachable!()
            }
        )
    }
    for i in [3,5,7,9] {
        waiting_spots[i] = Square::Forbidden;
    }
    let mut bins: Vec<Vec<Amphipod>> = vec![vec![], vec![], vec![], vec![]];
    let mut bin_capacity = 0;
    'outer: for line in lines {
        for pos in [3,5,7,9] {

            match line.chars().nth(pos) {
                Some('#') => break 'outer,
                Some('.') => {}
                Some('A') => {bins[(pos-1) / 2 -1].insert(0,Amphipod::Amber)},
                Some('B') => {bins[(pos-1) / 2 -1].insert(0,Amphipod::Bronze)},
                Some('C') => {bins[(pos-1) / 2 -1].insert(0,Amphipod::Copper)},
                Some('D') => {bins[(pos-1) / 2 -1].insert(0,Amphipod::Desert)},
                _ => unreachable!(),
            };
        }
        bin_capacity +=1;
    }

    // let bins = vec![vec![Amphipod::Amber,Amphipod::Bronze], vec![Amphipod::Desert, Amphipod::Copper], vec![Amphipod::Copper, Amphipod::Bronze], vec![Amphipod::Amber, Amphipod::Desert]];
    // let bin_capacity:isize = 2;

    let starting_state = GameState{ cost: 0, waiting_spots, bins};


    let mut heap = BinaryHeap::new();
    heap.push(starting_state);

    let bin_requirements: HashMap<usize,Amphipod> = HashMap::from([
        (0, Amphipod::Amber),
        (1, Amphipod::Bronze),
        (2, Amphipod::Copper),
        (3, Amphipod::Desert)]);

    let mut seen: HashSet<(Vec<Square>, Vec<Vec<Amphipod>>)> = HashSet::new();
    while let Some(state) = heap.pop() {

        if is_winning_position(&state.bins, bin_capacity as usize) {
            println!("Winner! {}", state.cost);
            return;
        } else if seen.contains(&(state.waiting_spots.clone(), state.bins.clone())) {
            continue;
        }
        seen.insert((state.waiting_spots.clone(), state.bins.clone()));
        
        for (i, n) in state.waiting_spots.iter().enumerate() {
            
            if let Square::Occupied(amphipod) = n {
                let target_bin_number = match &amphipod {
                    Amphipod::Amber => 0,
                    Amphipod::Bronze => 1,
                    Amphipod::Copper => 2,
                    Amphipod::Desert => 3,
                };
                let target_bin = state.bins.get(target_bin_number).unwrap();

                if target_bin.is_empty() || target_bin.iter().filter(|a| a != &amphipod).count() == 0 {

                    let mut reachable = true;
                    let target_index = (target_bin_number+1) *2 +1;
                    if i < target_index {
                        for n in i+1 .. target_index + 1 {
                            if state.waiting_spots[n] != Square::Forbidden && state.waiting_spots[n] != Square::Empty {
                                reachable = false;
                            }
                        }
                    } else {
                        for n in (target_index ..i ).rev() {
                            if state.waiting_spots[n] != Square::Forbidden && state.waiting_spots[n] != Square::Empty {
                                reachable = false;
                            }
                        }
                    }
                    
                    if reachable {
                        let steps =  ((target_bin_number as isize +1) * 2 + 1 - i as isize).abs() + (bin_capacity - target_bin.len() as isize);
                        let score = steps * match &amphipod {
                            Amphipod::Amber => 1,
                            Amphipod::Bronze => 10,
                            Amphipod::Copper => 100,
                            Amphipod::Desert => 1000,
                        };
                        let mut new_state = state.clone();
                        new_state.waiting_spots[i] = Square::Empty;
                        new_state.bins[target_bin_number].push(*amphipod);
                        new_state.cost += score;
                        heap.push(new_state);
                    }
                }
            }
        }

        for (i, mut bin) in state.bins.iter().enumerate() {
            if !bin.is_empty() {
                let top_item = bin.last().unwrap();
                if top_item != &bin_requirements[&i] || bin.iter().filter(|x| *x != &bin_requirements[&i]).count()  > 0 {
                    let corridor_point = (i+1) * 2 +1;
                    let mut potential_destinations: Vec<usize> = Vec::new();
                    for n in (1..corridor_point).rev() {
                        if state.waiting_spots[n] == Square::Empty {
                            potential_destinations.push(n);
                        } else if state.waiting_spots[n] != Square::Forbidden {
                            break;
                        }
                    }

                    for n in corridor_point..12 {
                        if state.waiting_spots[n] == Square::Empty {
                            potential_destinations.push(n);
                        } else if state.waiting_spots[n] != Square::Forbidden {
                            break;
                        }
                    }
                    for d in potential_destinations {
                        let mut new_state = state.clone();
                        let mut move_cost = bin_capacity - bin.len() as isize +1;
                        let moved = new_state.bins[i].pop().unwrap();
                        let distance = (corridor_point as isize - d as isize).abs();
                        move_cost += distance;
                        move_cost *=  match &moved {
                            Amphipod::Amber => 1,
                            Amphipod::Bronze => 10,
                            Amphipod::Copper => 100,
                            Amphipod::Desert => 1000,
                        };
                        new_state.waiting_spots[d] = Square::Occupied(moved);
                        new_state.cost += move_cost;
                        heap.push(new_state);
                    }

                } 
            }
        }


    }

    // println!("{:?}", heap);

}

fn is_winning_position(bins: &[Vec<Amphipod>], depth: usize) -> bool {
    // let depth = 2;
    bins[0].iter().filter(|a| **a == Amphipod::Amber).count() == depth &&
    bins[1].iter().filter(|a| **a == Amphipod::Bronze).count() == depth &&
    bins[2].iter().filter(|a| **a == Amphipod::Copper).count() == depth &&
    bins[3].iter().filter(|a| **a == Amphipod::Desert).count() == depth 
    
}