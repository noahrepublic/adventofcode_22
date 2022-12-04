use std::collections::HashMap;
use std::fs;

fn p1(content: &String) -> usize {
    let mut sum = 0;
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();
    for line in content.split("\n") {
        let rucksack = line.replace("\r", "").replace(" ", "");
        let (first, second) = rucksack.split_at(rucksack.len() / 2);

        for character in first.split("") {
            if character == "" {
                continue;
            }

            if second.contains(character) {
                let score = letter_scores
                    .get(&character.chars().next().unwrap())
                    .unwrap();

                sum += score;
                break;
               
            }
        }
    }
    return sum;
}

fn p2(content: &String) -> usize {
    let mut sum = 0;
    let _letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let mut lines = content.split("\n");
    for _i in 0..content.len()/3 {
        let first_rucksack = lines.next();
        if first_rucksack.is_none() {
            break;
        }
        let first_rucksack = &first_rucksack.unwrap().replace("\r", "").replace(" ", "");
        let second_rucksack = lines.next().unwrap().replace("\r", "").replace(" ", "");
        let third_rucksack = lines.next().unwrap().replace("\r", "").replace(" ", "");

        for character in first_rucksack.split("") {
            if character == "" {
                continue;
            }

            if second_rucksack.contains(character) && third_rucksack.contains(character) {
                let score = _letter_scores
                    .get(&character.chars().next().unwrap())
                    .unwrap();

                sum += score;
                break;
            }
        }
    }
    return sum;
}

fn main() {
    const FILE_PATH: &str = "../input_files/day_3.txt";

    let content = fs::read_to_string(FILE_PATH).unwrap();

    println!("Part 1: {}", p1(&content));

    println!("Part 2: {}", p2(&content));
}