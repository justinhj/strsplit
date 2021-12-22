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
        } else if !self.remainder.is_empty() {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        } else {
            None
        }
    }
}

fn main() {

    let input = StrSplit::new("a b c d e ", " ");
    let words = input.into_iter();

    for word in words {
        println!("Word! {}", word);
    }
}

#[cfg(test)] mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail() {
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }
}
