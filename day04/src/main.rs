use std::collections::HashMap;

#[derive(Debug,Hash,Eq,PartialEq, Copy, Clone)]
struct BingoNumber {
    marked: bool,
    location: Coordinate
}

#[derive(Debug,Hash,Eq,PartialEq, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let split_data = input.split("\n\n").collect::<Vec<_>>();
    let bingo_numbers = split_data[0].split(",").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
    let mut cards: Vec<HashMap<isize,BingoNumber>> = Vec::new();

    for card in &split_data[1..] {
        let mut parsed_card: HashMap<isize, BingoNumber> = HashMap::new();
        for (y, line) in card.split("\n").enumerate() {
            for (x, number) in line.split_ascii_whitespace().map(|n| n.parse::<isize>().unwrap()).enumerate() {
                parsed_card.insert(number, BingoNumber{location: Coordinate{x,y}, marked: false});
            }
        }
        cards.push(parsed_card);
    }
    play_bingo(&mut cards, &bingo_numbers, true);
}

fn play_bingo(cards: &mut Vec<HashMap<isize,BingoNumber>>, bingo_numbers: &Vec<isize>, first_winner: bool){
    let mut last_drawn_number: isize = -1;
    let mut score: isize = -1;
    let mut winning_card = usize::MAX;
    for number in bingo_numbers {
        last_drawn_number = *number;
        for card in cards.iter_mut() {
            if card.contains_key(&number) {
                card.get_mut(&number).unwrap().marked = true;
            }
        }
        for i in 0..cards.len() {
            if is_winner(&cards[i]) {
                winning_card = i;
                score = winning_score(&cards[i]);
            } else {

            }
        }
        if winning_card != usize::MAX {
            break;
        }
    }
    if first_winner {
        println!("Part 1 answer: {}", last_drawn_number * score);
    }
    if cards.len() > 1 {
        cards.swap_remove(winning_card);
        play_bingo(cards, bingo_numbers, false)
    } else {
        println!("Part 2 answer: {}", last_drawn_number * score);

    } 
}

fn is_winner(card: &HashMap<isize,BingoNumber>) -> bool {
    let mut coord_mappings: HashMap<Coordinate,bool> = HashMap::new();

    for (_, bingo_num) in card {
        coord_mappings.insert(bingo_num.location, bingo_num.marked);
    }

    for x in 0..5 {
        if (0..5).map(|y| *coord_mappings.get(&Coordinate{x,y}).unwrap()).all(|v| v) {
            return true;
        }
    }

    for y in 0..5 {
        if (0..5).map(|x| *coord_mappings.get(&Coordinate{x,y}).unwrap()).all(|v| v) {
            return true;
        }
    }    
    false
}

fn winning_score(card: &HashMap<isize,BingoNumber>) -> isize {
    let mut total: isize = 0;
    for (number, details) in card {
        if !details.marked {
            total += number;
        }
    }
    total
}