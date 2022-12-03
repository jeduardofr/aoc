use std::io;

// For reference:
// https://www.becomebetterprogrammer.com/rust-read-user-input-stdin/
// https://doc.rust-lang.org/stable/rust-by-example/std/vec.html
// https://www.cloudhadoop.com/rust-convert-string-int-example/
fn main() -> io::Result<()> {
    let mut elves_input: Vec<u32> = vec![];
    let stdin = io::stdin();
    let mut user_input = String::new();
    loop {
        let mut sum: u32 = 0;
        let mut should_break = false;
        loop {
            match stdin.read_line(&mut user_input) {
                Ok(bytes) => {
                    // EOF
                    if bytes == 0 {
                        should_break = true;
                        break;
                    }

                    // Carry line
                    if bytes == 1 {
                        break;
                    }

                    // We need to trim to remove \n
                    let value: u32 = user_input.trim().parse().expect("failed to read number");
                    sum += value;

                    // Truncate input, otherwise stdin.read_line concats to the previous contents
                    user_input.truncate(0);
                },
                // Just in case
                Err(err) => {
                    println!("error: {}", err);
                    should_break = true;
                    break
                }
            }
        }

        if sum > 0 {
            elves_input.push(sum);
        }

        if should_break {
            break;
        }
    }

    elves_input.sort();
    elves_input.reverse();

    println!("result 1: {}", elves_input[0]);
    println!("result 2: {}", elves_input[0] + elves_input[1] + elves_input[2]);

    Ok(())
}
