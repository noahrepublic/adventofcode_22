use std::fs;

fn main() {
    const FILE_PATH: &str = "../input_files/day_2.txt";

    let content = fs::read_to_string(FILE_PATH).unwrap();

    let mut your_score = 0;
    for line in content.split("\n") {
        let mut moves = line.split(" ");

        let outcome = match String::from(line.replace("\r", "").replace(" ", "")).as_str() {
            "AX" => "AZ",
            "BX" => "BX",
            "CX" => "CY",
            "AY" => "AX",
            "BY" => "BY",
            "CY" => "CZ",
            "AZ" => "AY",
            "BZ" => "BZ",
            "CZ" => "CX",
            _ => ""
        };
        

        let your_move = outcome.chars().nth(1).unwrap();

        let move_points = match your_move {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0
        };


        let did_win = match outcome {
            "AX" => 3,
            "BY" => 3,
            "CZ" => 3,
            "AY" => 6,
            "BZ" => 6,
            "CX" => 6,
            "AZ" => 0,
            "BX" => 0,
            "CY" => 0,
            _ => 0
        };

        your_score += move_points + did_win;

    }
    println!("Your score: {}", your_score);
}
