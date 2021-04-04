use std::{collections::LinkedList, iter::FromIterator, time::Instant};

pub struct Typer {
    typed_chars: LinkedList<char>,
    correct: LinkedList<char>,
    incorrect: LinkedList<char>,
    missing: LinkedList<char>,
    start: Instant,
    num_typed: usize,
    num_misses: usize,
}

impl Typer {
    pub fn new(inp: String) -> Self {
        Typer {
            typed_chars: LinkedList::new(),
            correct: LinkedList::new(),
            incorrect: LinkedList::new(),
            missing: LinkedList::from_iter(inp.chars()),
            start: Instant::now(),
            num_typed: 0,
            num_misses: 0,
        }
    }

    pub fn get_correct(&self) -> String {
        self.correct.iter().collect()
    }

    pub fn get_incorrect(&self) -> String {
        self.incorrect.iter().collect()
    }

    pub fn get_missing(&self) -> String {
        self.missing.iter().collect()
    }

    pub fn get_typed_chars(&self) -> String {
        self.typed_chars.iter().collect()
    }

    pub fn write(&mut self, c: char) -> () {
        self.typed_chars.push_back(c);
        self.num_typed += 1;

        if self.missing.is_empty() {
            return;
        }

        let next = self.missing.pop_front().unwrap();

        if self.incorrect.is_empty() && next == c {
            self.correct.push_back(next);
        } else {
            self.incorrect.push_back(next);
            self.num_misses += 1;
        }
    }

    pub fn remove_last(&mut self) -> () {
        self.typed_chars.pop_back();

        if self.typed_chars.len() >= self.inp_len() {
            return;
        }

        let last = if !self.incorrect.is_empty() {
            self.incorrect.pop_back()
        } else {
            self.correct.pop_back()
        };

        if let Some(c) = last {
            self.missing.push_front(c);
        }
    }

    fn inp_len(&self) -> usize {
        self.correct.len() + self.incorrect.len() + self.missing.len()
    }

    pub fn is_finished(&self) -> bool {
        self.correct.len() == self.inp_len()
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    pub fn wpm(&self) -> f64 {
        let num_words = self
            .get_correct()
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .len();

        let elapsed_secs = self.elapsed_secs();
        if elapsed_secs == 0.0 {
            0.0
        } else {
            num_words as f64 / (elapsed_secs / 60.0)
        }
    }

    pub fn accuracy(&self) -> f64 {
        1.0 - if self.num_misses == 0 {
            0.0
        } else {
            self.num_misses as f64 / self.num_typed as f64
        }
    }
}
