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
                let my_secret_move = mine.trim();

                let vs_score: u32 = match opponent_move {
                    "A" => match my_secret_move {
                        "Y" => 6,
                        "X" => 3,
                        _ => 0,
                    },
                    "B" => match my_secret_move {
                        "Z" => 6,
                        "Y" => 3,
                        _ => 0,
                    },
                    "C" => match my_secret_move {
                        "X" => 6,
                        "Z" => 3,
                        _ => 0,
                    },
                    _ => 0,
                };

                let move_score: u32 = match my_secret_move {
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
