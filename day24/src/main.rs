use hashbrown::HashMap;

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut registers: HashMap<&str, isize> = HashMap::new();
    let mut input ="11211791111365".chars().map(|x| x.to_digit(10).unwrap() as isize).rev().collect::<Vec<_>>();
    // println!("{:?}", input);
    registers.insert("w", 0);
    registers.insert("x", 0);
    registers.insert("y", 0);
    registers.insert("z", 0);

    for line in data.split('\n') {

        let words = line.split_ascii_whitespace().collect::<Vec<_>>();
        let mut b = 0;
        if words[0] != "inp" {
            if let Ok(n) = words[2].parse::<isize>() {
                b = n;
            } else {
                b = *registers.get(words[2]).unwrap();
            }
        }
    
        match words[0]  {
            "inp" => {
                registers.insert(words[1], input.pop().unwrap());
                println!("{:?}", registers);
            },
            "add" => {registers.insert(words[1], registers[words[1]] + b);}
            "mul" => {registers.insert(words[1], registers[words[1]] * b);}
            "div" => {registers.insert(words[1], registers[words[1]] / b);}
            "mod" => {registers.insert(words[1], registers[words[1]] % b);}
            "eql" => {registers.insert(words[1], if registers[words[1]] == b {1} else {0});}
            _ => {}
        }
    }
    println!("{:?}", registers);
}

