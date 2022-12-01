use std::fs;

fn main() {
    const FILE_PATH: &str = "../input_files/day_1/day1_1.txt";

    let content = fs::read_to_string(FILE_PATH).unwrap();

    let calories = content.split("\n");
    let mut elves: [i32; 3] = [0, 0, 0];

    let mut elf_calories = 0;
    for line in calories {
        let mut string_line :String = String::from(line);
        if string_line.trim().is_empty() {
            elves.sort();

            for i in 0..elves.len() {
                if elves[i] < elf_calories {
                    elves[i] = elf_calories;
                    break;
                }
            }

            elf_calories = 0;
            continue;
        }
        string_line.pop();
        let calories_num = string_line.parse::<i32>().unwrap();
        elf_calories += calories_num;
    }

    elves.sort();
    println!("3 highest snack counts (total): {}", elves[0] + elves[1] + elves[2]);

}
