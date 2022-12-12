use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn calculate_score(outcome: i32, opponent_play: i32)-> i32{

    // -1 for lose, 0 for draw 1 for win

    let mut diff = outcome + opponent_play;
    let mut own_play = diff;

    if own_play > 2 {
        own_play = 0;
    } else if own_play < 0 {
        own_play = 2
    }

    println!("own_play {} opponent_play {} outcome {} score: {}", own_play, opponent_play, outcome, 1 + own_play + 3 * (outcome + 1));
    1 + own_play + 3 * (outcome + 1)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut indexMap = HashMap::new();
    indexMap.insert('A', 0);
    indexMap.insert('B', 1);
    indexMap.insert('C', 2);
    indexMap.insert('X', -1);
    indexMap.insert('Y', 0);
    indexMap.insert('Z', 1);

    let mut score = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        println!("line: {}", l);
        let first = l.chars().nth(0).unwrap();
        let second = l.chars().nth(2).unwrap();

        score += calculate_score(*indexMap.get(&second).unwrap(), *indexMap.get(&first).unwrap());
    }

    println!("total score {}", score);
}