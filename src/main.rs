use std::env;
use std::io::{BufRead, BufReader, Read, stdin};
use std::fs::File;
use std::collections::HashMap;


fn main() {

    // read the training corpus from a file
    let training_filename = env::args().nth(1).expect("Usage: cargo run <training_file>");
    let mut file = File::open(training_filename).expect("Could not read file");
    let training_words = read_words(file);

    // TODO: pass words into our model for training
    println!("Model trained!");
    println!("Enter words to be corrected (ctrl+C to quit):");
    loop {

        let mut input = String::new();
        // TODO: pass the word into trained model and correct if necessary
        stdin().read_line(&mut input).expect("Could not read line!");

    }

}


fn read_words<R: Read>(reader: R) -> Vec<(String, usize)> {
    //! reads words from stdin and outputs a vector tuple containing the words
    //! and their frequencies (sorted by frequency)
    //! delimited by spaces

    let mut lines = BufReader::new(reader).lines();
    let mut words = HashMap::new();

    while let Some(Ok(line)) = lines.next() {

        let mut split_line = line.split_whitespace();
        for s in split_line {
            if words.contains_key(s) {
                *words.get_mut(s).unwrap() += 1;
            } else {
                words.insert(s.to_owned(), 1);
            }
        }
    }

    let mut sorted_words: Vec<(String, usize)> = words.into_iter().collect();
    sorted_words.sort_by(|w1, w2| (w2.1).cmp(&(w1.1))); // Sort by frequency

    sorted_words
}

#[cfg(test)]
mod read_words_tests {

    use super::read_words;
    use std::io::Cursor;

    #[test]
    fn reads() {
        assert_read(vec![("hello".to_owned(), 1)], "hello\n");
    }

    #[test]
    fn sorts_by_freq() {
        assert_read(vec![("hello".to_owned(), 3), ("world".to_owned(), 2)],
            "world\nhello\nworld\nhello\nhello\n");
    }

    #[test]
    fn splits_spaces() {
        assert_read(vec![("hello".to_owned(), 3), ("world".to_owned(), 2)],
            "world\thello\n\tworld   hello\t\n\nhello\n");
    }

    //fn assert_read(expected: &[&str], input: &str) {
    fn assert_read(expected: Vec<(String, usize)>, input: &str) {
        let mock_read = Cursor::new(input);
        let words = read_words(mock_read);
        assert_eq!(expected.to_owned(), words);
    }

}
