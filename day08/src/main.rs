use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut counter = 0;
    let mut total = 0;
    for line in input.split("\n") {
        let mut number_mappings: HashMap<i32,HashSet<char>> = HashMap::new();
        let mut split = line.split(" | ");
        let input_section = split.next().unwrap();
        let output_section = split.next().unwrap();
        for out in output_section.split(" ") {
            if out.len() == 2 || out.len() == 3 || out.len() == 4 || out.len() == 7  {
                counter +=1;
            } if out.len() == 2 {

            }
        }
        let mut unconsidered = input_section.split(" ").collect::<VecDeque<_>>();
        while number_mappings.len() < 10 {
            let word = unconsidered.pop_front().unwrap();
                match word.len() {
                    2 => {
                        number_mappings.insert(1, word.chars().collect::<HashSet<_>>());
                    }
                    3 =>  {
                        number_mappings.insert(7, word.chars().collect::<HashSet<_>>());
                    },
                    4 => {
                       number_mappings.insert(4, word.chars().collect::<HashSet<_>>());
                    },

                    5 => {
                        // Could be 2, 3 or 5
                        if number_mappings.contains_key(&9) {
                            let word_set = word.chars().collect::<HashSet<_>>();
                            if word_set.union(number_mappings.get(&9).unwrap()).count() == 7 {
                                // if it doesn't completely overlap 9, it's a 2
                                number_mappings.insert(2, word.chars().collect::<HashSet<_>>());
                            } else {
                                if number_mappings.contains_key(&1) {
                                    if word_set.union(number_mappings.get(&1).unwrap()).count() == 6 {
                                        // if doesn't completely overlap with 1, it's a 5
                                        number_mappings.insert(5, word.chars().collect::<HashSet<_>>());
                                    } else {
                                        number_mappings.insert(3, word.chars().collect::<HashSet<_>>());

                                    }
                               } else {
                                    unconsidered.push_back(word)
                                }
                            }
                        } else {
                            unconsidered.push_back(word)
                        }
                    },
                    6 => {
                        // Could be 0, 6 or 9
                        if number_mappings.contains_key(&1) {
                            let word_set = word.chars().collect::<HashSet<_>>();
                            if word_set.union(number_mappings.get(&1).unwrap()).count() == 7 {
                                // if it doesn't completely overlap with 1, it's a 6
                                number_mappings.insert(6, word.chars().collect::<HashSet<_>>());
                            } else {
                                if number_mappings.contains_key(&4) {
                                    if word_set.union(number_mappings.get(&4).unwrap()).count() == 7 {
                                        // if it doesn't completely overlap with 4, it's a 0
                                        number_mappings.insert(0, word.chars().collect::<HashSet<_>>());
                                    } else {
                                        // it's a 9
                                        number_mappings.insert(9, word.chars().collect::<HashSet<_>>());
                                    }
                                } else {
                                    unconsidered.push_back(word)
                                }
                                    
                            }
                        } else {
                            unconsidered.push_back(word)
                        }
                    }

                    7 => {
                        // it's a 8
                        number_mappings.insert(8, word.chars().collect::<HashSet<_>>());
                    }

                    _ => {}
                
            
            }
        }
            let mut output = 0;
            for word in output_section.split(" ") {
                let char_set = word.chars().collect::<HashSet<_>>();
                for num in number_mappings.keys() {
                    if char_set == *number_mappings.get(num).unwrap() {
                        output = (output*10) + num;
                        break;
                    }
                }
            }
            total += output;
    }
    println!("Part 1: {}", counter);
    println!("Part 2: {}", total);

}

