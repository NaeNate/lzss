use std::{env, fs};

const SEARCH_BUFFER_SIZE: usize = 10;
const LOOK_AHEAD_BUFFER_SIZE: usize = 5;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).unwrap();
    let mut chars = contents.chars();

    if args[1].split(".").last().unwrap() == "lzss" {
        //
    } else {
        let mut encoded = String::new();

        let mut search_buffer: Vec<char> = Vec::new();
        let mut look_ahead_buffer = Vec::new();

        let first = chars.next().unwrap();

        search_buffer.push(first);
        encoded.push(first);

        for _ in 0..LOOK_AHEAD_BUFFER_SIZE {
            look_ahead_buffer.push(chars.next().unwrap());
        }

        for _ in 0..400 {
            let mut offset = 0;
            let mut length = 1;

            loop {
                if let Some(index) = search_buffer
                    .windows(length)
                    .position(|w| w == &look_ahead_buffer[0..length])
                {
                    offset = SEARCH_BUFFER_SIZE - index;
                    length += 1;
                } else {
                    break;
                }
            }

            if length > 2 {
                for _ in 0..length - 1 {
                    let char = look_ahead_buffer.remove(0);
                    look_ahead_buffer.push(chars.next().unwrap());

                    search_buffer.push(char);

                    if search_buffer.len() > SEARCH_BUFFER_SIZE {
                        search_buffer.remove(0);
                    }
                }

                encoded.push_str(&format!("({},{})", offset, length - 1));
            } else {
                let char = look_ahead_buffer.remove(0);
                look_ahead_buffer.push(chars.next().unwrap());

                search_buffer.push(char);

                if search_buffer.len() > SEARCH_BUFFER_SIZE {
                    search_buffer.remove(0);
                }

                encoded.push(char);
            }
        }

        fs::write("out.lzss", encoded).unwrap();
    }
}
