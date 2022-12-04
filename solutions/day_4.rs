use std::fs;

fn p1(content: &String) -> i32 {
    let mut amount = 0;
    for line in content.split("\n") {
        let mut split = line.split(',');
        let range1 = split.next().unwrap();
        let range2 = split.next().unwrap();

        let binding = range1.split('-').collect::<Vec<&str>>();
        let (min_1, max_1) = &binding.as_slice().split_at(1);

        let binding = range2.split('-').collect::<Vec<&str>>();
        let (min_2, max_2) = &binding.as_slice().split_at(1);
        
        if min_1 <= min_2 && max_1 >= max_2 || min_1 >= min_2 && max_1 <= max_2 {
            amount += 1;
            continue;
        }
    }
    return amount;
}

fn p2(content: &String) -> i32 {
    let mut amount = 0;
    for line in content.split("\n") {
        let mut split = line.split(',');
        let range1 = split.next().unwrap().replace("\r", "");
        let range2 = split.next().unwrap().replace("\r", "");

        let binding = range1.split('-').collect::<Vec<&str>>();
        let (min_1, max_1) = &binding.as_slice().split_at(1);
        let min_1 = min_1[0].parse::<i32>().unwrap();
        let max_1 = max_1[0].parse::<i32>().unwrap();

        let binding = range2.split('-').collect::<Vec<&str>>();
        let (min_2, max_2) = &binding.as_slice().split_at(1);
        let min_2 = min_2[0].parse::<i32>().unwrap();
        let max_2 = max_2[0].parse::<i32>().unwrap();
        
        if min_1 <= min_2 && min_2 <= max_1 || min_1 <= max_2 && max_2 <= max_1 || min_2 <= min_1 && min_1 <= max_2 || min_2 <= max_1 && max_1 <= max_2 {
            amount += 1;
        }
    }
    return amount;
}

fn main() {
    const FILE_PATH: &str = "../input_files/day_4.txt";

    let content = fs::read_to_string(FILE_PATH).unwrap();

    println!("Part 1: {}", p1(&content));

    println!("Part 2: {}", p2(&content));
}
