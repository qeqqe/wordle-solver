use std::time::{SystemTime, UNIX_EPOCH};

struct Wordle {
    word: Vec<u8>,
    guesses: Vec<Guesses>,
}

struct Guesses {
    guessed_word: Vec<u8>,
    //                index, word
    right_position: Vec<(u8, u8)>,
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
            guesses: Vec::<Guesses>::new(),
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
    let word = Wordle::new();
    println!("{:?}", word.word);
}

