use std::collections::{HashMap, BTreeSet};

pub struct Tokenizer {
    stoi: HashMap<char, usize>,
    itos: HashMap<usize, char>,
}

impl Tokenizer {
    pub fn new(text: &str) -> Tokenizer {
        let mut charset = BTreeSet::new();
        charset.extend(text.chars());

        let mut stoi: HashMap<char, usize> = HashMap::new();
        let mut itos = HashMap::new();

        for (i, c) in charset.iter().enumerate() {
            stoi.insert(*c, i);
            itos.insert(i, *c);
        }

        Tokenizer {
            stoi,
            itos
        }
    }

    pub fn encode(&self, input: &str) -> Vec<usize> {
        // input.chars().map(|c| self.stoi.get(&c).unwrap_or(self.stoi.len())).collect()
        input.chars().map(|c| self.stoi[&c]).collect()
    }

    pub fn decode(&self, input: &[usize]) -> String {
        input.iter().map(|i| self.itos[i]).collect()
    }
}