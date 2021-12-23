#![warn(rust_2018_idioms, dead_code)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str
}

impl <'a>StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        StrSplit{
                remainder: Some(haystack),
                delimiter}
    }
}

impl <'a>Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.remainder {
            None => None,
            Some(remainder) => {
                if let Some(next_delim) = remainder.find(self.delimiter) {
                   let until_next = &remainder[..next_delim];
                   self.remainder = Some(&remainder[(next_delim + self.delimiter.len())..]);
                   Some(until_next)
                } else {
                    self.remainder = None;
                    Some(remainder)
                }
            }
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
