use std::env;
use std::io::{BufRead, BufReader, Read, stdin};
use std::fs::File;


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


fn read_words<R: Read>(reader: R) -> Vec<String> {
    //! reads words from stdin and out puts a vector containing the words
    //! delimited by spaces

    let mut words: Vec<String> = vec![];

    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {

        let mut split_line = line.split(" ");
        for s in split_line {
            words.push(s.to_string());
        }
        // words.push(line.trim().to_string());
    }

    words

}

#[cfg(test)]
mod read_words_tests {

    use super::read_words;
    use std::io::Cursor;

    #[test]
    fn reads() {
        assert_read(&["hello", "world"], "hello\nworld\n");
    }

    fn assert_read(expected: &[&str], input: &str) {
        let mock_read = Cursor::new(input);
        let words = read_words(mock_read);
        assert_eq!(expected.to_owned(), words);
    }

}
