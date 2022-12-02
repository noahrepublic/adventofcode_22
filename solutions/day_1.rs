use std::fs;

fn main() {
    const FILE_PATH: &str = "../input_files/day_1/day_1.txt";

    let content = fs::read_to_string(FILE_PATH).unwrap();
    let mut elves = Vec::new();

    let mut elf_calories = 0;
    for line in content.split("\n") {
        let mut string_line :String = String::from(line);
        if string_line.trim().is_empty() {
            elves.push(elf_calories);

            elf_calories = 0;
            continue;
        }
        string_line.pop();
        let calories_num = string_line.parse::<i32>().unwrap();
        elf_calories += calories_num;
    }

    elves.sort();
    elves.reverse();

    println!("3 highest snack counts (total): {}", elves[0] + elves[1] + elves[2]);

}
