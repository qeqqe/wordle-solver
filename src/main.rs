use std::{
    collections::HashSet,
    time::{SystemTime, UNIX_EPOCH},
};

struct Wordle {
    word: Vec<u8>,
    guesses: Guesses,
    word_freq: [u8; 26],
    dictionary: HashSet<&'static str>,
}

#[derive(Default)]
struct Guesses {
    wrong: HashSet<u8>,
    //                index, word
    right_position: HashSet<(u8, u8)>,
    // just store the alpahabets
    wrong_position: HashSet<(u8, u8)>,
}

#[derive(PartialEq, Clone, Copy)]
enum LetterResult {
    Match,
    Exist,
    Miss,
}

impl Wordle {
    fn new() -> Self {
        let index = Self::generate_random_index() as usize;
        println!("index: {}", index);
        let word = Self::get_word(index).unwrap();

        let mut word_freq: [u8; 26] = [0; 26];
        for ch in &word {
            let index = (ch - 97) as usize;
            word_freq[index] += 1;
        }
        let dictionary = Self::load_words();
        Self {
            word,
            guesses: Guesses::default(),
            word_freq,
            dictionary,
        }
    }

    pub fn play_loop(&mut self) {
        let target_display = String::from_utf8_lossy(&self.word);
        println!("Actual Word: {}", target_display);

        let mut attempt = 1;

        while attempt <= 6 {
            println!("\ntry number: {}", attempt);

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("failed read");
            let input = input.trim().to_string();

            if !self.dictionary.contains(input.as_str()) {
                println!("Word doesn't exit, try again...");
                continue;
            }

            let guess: Vec<u8> = input.trim().as_bytes().to_vec();

            if guess.len() != 5 {
                println!("need 5 letters, got {}", guess.len());
                continue;
            }

            let mut freq_clone = self.word_freq;
            let mut result: [LetterResult; 5] = [LetterResult::Miss; 5];

            // for matching
            for ind in 0..5 {
                if guess[ind] == self.word[ind] {
                    freq_clone[(guess[ind] - b'a') as usize] -= 1;
                    result[ind] = LetterResult::Match;
                }
            }

            // for existing
            for ind in 0..5 {
                if result[ind] == LetterResult::Match {
                    continue;
                }
                let remain_ind = (guess[ind] - b'a') as usize;
                if freq_clone[remain_ind] > 0 {
                    result[ind] = LetterResult::Exist;
                    freq_clone[remain_ind] -= 1;
                }
            }

            let mut matches = 0;

            for ind in 0..5 {
                let g_char = guess[ind] as char;
                let g_ascii = guess[ind];
                match result[ind] {
                    LetterResult::Match => {
                        println!("[MATCH] index {}: '{}'", ind, g_char);
                        self.guesses.right_position.insert((ind as u8, g_ascii));
                        matches += 1;
                    }
                    LetterResult::Exist => {
                        println!("[EXIST] on {}: '{}'", ind, g_char);
                        self.guesses.wrong_position.insert((ind as u8, g_ascii));
                    }
                    LetterResult::Miss => {
                        println!("[MISS ] brah {}: '{}'", ind, g_char);
                        self.guesses.wrong.insert(g_ascii);
                    }
                }
                println!(
                    "Stats. Right position {:?}, Wrong position {:?}, Wrong {:?}",
                    self.guesses.right_position, self.guesses.wrong_position, self.guesses.wrong
                );
            }
            if matches == 5 {
                println!("\nDone it in {} tries!", attempt);
                break;
            }
            attempt += 1;
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

    fn load_words() -> HashSet<&'static str> {
        let content = include_str!("../wordle-dictionary.txt");
        content.lines().collect()
    }
}

fn main() {
    let mut word = Wordle::new();
    word.play_loop();
}
