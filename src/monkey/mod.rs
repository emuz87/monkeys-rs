static ALPHA: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

use rand::{seq::SliceRandom, rngs::ThreadRng};

pub struct Monkey<'a> {
    rng: ThreadRng,
    string: &'a str,
    string_length: usize,
    attempts: u64
}
impl Iterator for Monkey<'_> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.attempts+=1;
        let mut buf = String::new();
        (0..self.string_length).into_iter()
            .for_each(|_| buf.push(*ALPHA.choose(&mut self.rng).unwrap()));
        Some(buf)
    }
}
impl<'a> Monkey<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            rng: rand::thread_rng(),
            string,
            string_length: string.len(),
            attempts: 0
        }
    }
    pub fn compute_attempts(&mut self) -> u64 {
        self.attempts = 0;
        let string = self.string;
        self.find(|cmp_string| string == cmp_string.as_str());
        self.attempts
    }
}