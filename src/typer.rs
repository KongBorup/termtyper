use std::{collections::LinkedList, iter::FromIterator};

pub struct Typer {
    typed_chars: LinkedList<char>,
    correct: LinkedList<char>,
    incorrect: LinkedList<char>,
    missing: LinkedList<char>,
}

impl Typer {
    pub fn new(inp: String) -> Self {
        Typer {
            typed_chars: LinkedList::new(),
            correct: LinkedList::new(),
            incorrect: LinkedList::new(),
            missing: LinkedList::from_iter(inp.chars()),
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

        if self.missing.is_empty() {
            return;
        }

        let next = self.missing.pop_front().unwrap();

        if self.incorrect.is_empty() && next == c {
            self.correct.push_back(next);
        } else {
            self.incorrect.push_back(next);
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
}
