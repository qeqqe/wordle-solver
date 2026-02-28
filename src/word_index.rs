pub enum ConstraintKind {
    GreenAt(usize),
    YellowAt(usize),
    Gray,
}

pub struct Constraint {
    pub letter: u8,
    pub kind: ConstraintKind,
}

pub struct WordIndex {
    words: Vec<&'static str>,
    bitset_len: usize,
    pos_letter: Vec<Vec<u64>>,
    contains: Vec<Vec<u64>>,
}

impl WordIndex {
    pub fn build(words: Vec<&'static str>) -> Self {
        let n = words.len();
        let bitset_len = (n + 63) / 64;

        let mut pos_letter = vec![vec![0u64; bitset_len]; 5 * 26];
        let mut contains = vec![vec![0u64; bitset_len]; 26];

        for (i, word) in words.iter().enumerate() {
            let chunk = i / 64;
            let bit = i % 64;
            let bytes = word.as_bytes();

            let mut seen = 0u32;
            for pos in 0..5usize {
                let l = (bytes[pos] - b'a') as usize;
                pos_letter[pos * 26 + l][chunk] |= 1 << bit;
                if seen & (1 << l) == 0 {
                    seen |= 1 << l;
                    contains[l][chunk] |= 1 << bit;
                }
            }
        }

        Self {
            words,
            bitset_len,
            pos_letter,
            contains,
        }
    }

    pub fn query(&self, constraints: &[Constraint]) -> Vec<&'static str> {
        let n = self.words.len();
        let mut result = vec![!0u64; self.bitset_len];

        let rem = n % 64;
        if rem != 0 {
            result[self.bitset_len - 1] = (1u64 << rem) - 1;
        }

        for c in constraints {
            let l = (c.letter - b'a') as usize;
            match c.kind {
                ConstraintKind::GreenAt(pos) => {
                    let mask = &self.pos_letter[pos * 26 + l];
                    for i in 0..self.bitset_len {
                        result[i] &= mask[i];
                    }
                }
                ConstraintKind::YellowAt(pos) => {
                    let contains_mask = &self.contains[l];
                    let at_pos_mask = &self.pos_letter[pos * 26 + l];
                    for i in 0..self.bitset_len {
                        result[i] &= contains_mask[i] & !at_pos_mask[i];
                    }
                }
                ConstraintKind::Gray => {
                    let mask = &self.contains[l];
                    for i in 0..self.bitset_len {
                        result[i] &= !mask[i];
                    }
                }
            }
        }

        let mut out = Vec::new();
        for (chunk_idx, mut chunk) in result.into_iter().enumerate() {
            while chunk != 0 {
                let bit = chunk.trailing_zeros() as usize;
                let idx = chunk_idx * 64 + bit;
                if idx < n {
                    out.push(self.words[idx]);
                }
                chunk &= chunk - 1;
            }
        }
        out
    }
}

