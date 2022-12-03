use std::collections::HashMap;
use std::fs;

fn main() {
    const FILE_PATH: &str = "../input_files/day_3.txt";

    let content = fs::read_to_string(FILE_PATH).unwrap();

    let mut sum = 0;
    let ascii_a = 'a' as u8;
    println!("a: {}", ascii_a);
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
                println!(
                    "{}",
                    letter_scores
                        .get(&character.chars().next().unwrap())
                        .unwrap()
                );
                let score = letter_scores
                    .get(&character.chars().next().unwrap())
                    .unwrap();

                sum += score;
            }
        }
    }
    println!("Sum: {}", sum);
}
