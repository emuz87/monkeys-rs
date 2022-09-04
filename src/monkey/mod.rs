use rand::{Rng, rngs::ThreadRng};

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
            .for_each(|_| buf.push(self.rng.gen::<u8>() as char));
        Some(buf)
    }
}
impl<'a> Monkey<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            rng: rand::thread_rng(),
            string: string,
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