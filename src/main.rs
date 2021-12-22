#![warn(rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str
}

impl <'a>StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        StrSplit{
                remainder: haystack,
                delimiter}
    }
}

impl <'a>Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
           let until_next = &self.remainder[..next_delim];
           self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
           Some(until_next)
        } else {
            self.remainder = "";
            None
        }
    }
}

fn main() {

    println!("Yes");

    let input = StrSplit::new("a b c d empty full poop", " ");

    let words = input.into_iter();

    for word in words {
        println!("Word! {}", word);
    }
}
