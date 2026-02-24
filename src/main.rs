use std::{
    collections::HashSet,
    default,
    io::stdin,
    time::{SystemTime, UNIX_EPOCH},
};

struct Wordle {
    word: Vec<u8>,
    guesses: Guesses,
}

#[derive(Default)]
struct Guesses {
    wrong: Vec<u8>,
    //                index, word
    right_position: HashSet<(u8, u8)>,
    // just store the alpahabets
    wrong_position: Vec<u8>,
}

impl Wordle {
    fn new() -> Self {
        let index = Self::generate_random_index() as usize;
        println!("index: {}", index);
        let word = Self::get_word(index).unwrap();
        Self {
            word,
            guesses: Guesses::default(),
        }
    }

    pub fn play_loop(&mut self) {
        let target_display = String::from_utf8_lossy(&self.word);
        println!("Secret Word: {}", target_display);

        for attempt in 1..=6 {
            println!("\n--- ATTEMPT {} ---", attempt);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed read");

            let guess: Vec<u8> = input.trim().as_bytes().to_vec();

            if guess.len() != 5 {
                println!("Error: Need exactly 5 letters, got {}", guess.len());
                continue;
            }
            let mut matches = 0;
            for ind in 0..5 {
                let g_char = guess[ind] as char;

                if guess[ind] == self.word[ind] {
                    println!("[MATCH] Index {}: '{}' is in the right spot!", ind, g_char);
                    self.guesses.right_position.insert((ind as u8, guess[ind]));
                    matches += 1;
                } else if self.word[ind..].contains(&guess[ind]) {
                    println!(
                        "[EXIST] Index {}: '{}' exists later in the word.",
                        ind, g_char
                    );
                    self.guesses.wrong_position.push(guess[ind]);
                } else {
                    println!("[MISS ] Index {}: '{}' is not there.", ind, g_char);
                    self.guesses.wrong.push(guess[ind]);
                }
            }
            if matches == 5 {
                break;
            }
        }
    }
    fn generate_random_index() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            % 10656
    }

    fn get_word(ind: usize) -> Option<Vec<u8>> {
        let contents = include_bytes!("../wordle-dictionary.txt");
        contents.split(|&b| b == b'\n').nth(ind).map(|l| l.to_vec())
    }
}

fn main() {
    let mut word = Wordle::new();
    word.play_loop();
}

