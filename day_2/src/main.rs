use std::io;

// X -> Rock
// Y -> Paper
// Z -> Scissors
//
// A -> Rock
// B -> Paper
// C -> Scissors
//
// A > C
// B > A
// C > B

// enum Points {
//     Rock = 1,
//     Paper = 2,
//     Scissors = 3,
// }

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut user_input = String::new();

    let mut my_score: u32 = 0;

    loop {
        match stdin.read_line(&mut user_input) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }

                let sanitized = user_input.trim();
                let index = sanitized.find(' ').unwrap();
                let (op, mine) = sanitized.split_at(index);

                let opponent_move = op.trim();
                let what_should_i_do = mine.trim();
                let mut my_secret_move = String::new();

                let vs_score: u32 = match opponent_move {
                    "A" => match what_should_i_do {
                        "X" => {
                            my_secret_move = String::from("Z");
                            0
                        },
                        "Y" => {
                            my_secret_move = String::from("X");
                            3
                        },
                        "Z" => {
                            my_secret_move = String::from("Y");
                            6
                        },
                        _ => 0,
                    },
                    "B" => match what_should_i_do {
                        "X" => {
                            my_secret_move = String::from("X");
                            0
                        },
                        "Y" => {
                            my_secret_move = String::from("Y");
                            3
                        },
                        "Z" => {
                            my_secret_move = String::from("Z");
                            6
                        },
                        _ => 0,
                    },
                    "C" => match what_should_i_do {
                        "X" => {
                            my_secret_move = String::from("Y");
                            0
                        },
                        "Y" => {
                            my_secret_move = String::from("Z");
                            3
                        },
                        "Z" => {
                            my_secret_move = String::from("X");
                            6
                        },
                        _ => 0,
                    },
                    _ => 0,
                };

                let move_score: u32 = match my_secret_move.as_ref() {
                    "Y" => 2,
                    "X" => 1,
                    "Z" => 3,
                    _ => 0,
                };

                my_score += move_score + vs_score;
                user_input.truncate(0);
            },
            Err(err) => {
                println!("error: {}", err);
                break
            }
        }
    }

    println!("score: {}", my_score);

    Ok(())
}
