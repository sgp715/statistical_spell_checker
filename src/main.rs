use std::env;
use std::io::{BufRead, BufReader, Read, stdin};
use std::fs::File;
use std::collections::HashMap;


// going of the implementation in the link
// http://norvig.com/spell-correct.html


fn main() {

    // read the training corpus from a file
    let training_filename = env::args().nth(1).expect("Usage: cargo run <training_file>");
    let file = File::open(training_filename).expect("Could not read file");
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
    //! reads words from stdin and outputs a vector tuple containing the words
    //! and their frequencies (sorted by frequency)
    //! delimited by spaces

    let mut words: Vec<String> = vec![];

    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {

        let split_line = line.split_whitespace();

        for s in split_line {
            words.push(s.to_string());
        }
    }

    words

}

#[cfg(test)]
mod read_words_tests {

    use super::read_words;

    // fn assert_read(expected: HashMap<String, i64>, input: &str) {
    //
    //     let mock_read = Cursor::new(input);
    //     let words = words_count(mock_read);
    //     assert_eq!(expected.to_owned(), words);
    //
    // }

}


fn counter(words: &Vec<String>) -> HashMap<String, i64> {

    //! takes in a vector of words creates a hashmap with the frequencies
    //! of the words from the vector

    let mut frequencies: HashMap<String, i64> = HashMap::new();

    for w in words {

        // fixed this hopefully?
        let count: i64 = *frequencies.entry(w.to_string()).or_insert(0);
        frequencies.insert(w.to_string(), count + 1);

    }

    frequencies

}

#[cfg(test)]
mod counter_tests {

    use super::counter;
    use std::io::Cursor;
    use std::collections::HashMap;

    #[test]
    fn simple() {

        let mut words: Vec<String> = vec![];
        words.push("hello".to_string());
        words.push("world".to_string());
        words.push("world".to_string());

        let expected = counter(&words);

        assert_eq!(expected.contains_key("world"), true);
        assert_eq!(expected["world"], 2);

        assert_eq!(expected.contains_key("hello"), true);
        assert_eq!(expected["hello"], 1);

    }

}

fn split(words: &str) -> Vec<String> {

    let splits: Vec<String> = vec![];

    splits

}

#[cfg(test)]
mod test_split {

    use super::split;

    #[test]
    fn blank_test() {

        let word = "";

        let expected: Vec<String> = vec![];
        let actual = split(word);

        // lengths should be equal
        assert_eq!(actual.len(), expected.len());

        // the elements should be equal
        assert_eq!(actual, expected);

    }

    #[test]
    fn simple() {

        let word = "test";

        let mut expected: Vec<String> = vec![];
        expected.push("t".to_string());
        expected.push("est".to_string());
        expected.push("te".to_string());
        expected.push("st".to_string());
        expected.push("tes".to_string());
        expected.push("t".to_string());
        let actual = split(word);

        assert_eq!(actual, expected);

    }

}


fn edits1(words: &Vec<String>) -> Vec<String> {

    let mut edits1_words: Vec<String> = vec![];

    let letters = "abcdefghijklmnopqrstuvwxyz";

    // gets all the splitted words
    let mut splits: Vec<String> = vec![];
    for w in words {
         splits.append(&mut split(w));
    }

    let mut deletes: Vec<String> = vec![];
    let mut transposes: Vec<String> = vec![];
    let mut replaces: Vec<String> = vec![];
    let mut inserts: Vec<String> = vec![];


    edits1_words

}

#[cfg(test)]
mod edits1_tests {

}


fn edits2(words: &Vec<String>) -> Vec<String> {

    let mut edits2_words: Vec<String> = vec![];

    edits2_words

}

#[cfg(test)]
mod edits2_tests {

}


fn known(words: Vec<String>) -> Vec<String> {

    let mut known_words: Vec<String> = vec![];

    known_words

}

#[cfg(test)]
mod known_tests {

}


fn candidates(words: Vec<String>) -> Vec<String> {

    let mut candidates_words: Vec<String> = vec![];

    candidates_words

}

#[cfg(test)]
mod candidates_tests {

}


fn correction(words: Vec<String>) -> Vec<String> {

    let mut correction_words: Vec<String> = vec![];

    correction_words

}

#[cfg(test)]
mod correction_tests {

}





































// fn word_probability(possible_words: &Vec<String>,
//                 trained_words: &HashMap<String, i64>) -> String {
//     //! Given a vector of possible words, return the word that has the highest
//     //! probability based on the training set. Return "-" if no possible words
//     //! are found in the training set.
//
//     let mut best_word = String::from("-");
//     let mut max = 0;
//
//     'possible: for p_word in possible_words {
//         'trained: for train in trained_words {
//             let (ref word, freq) = *train;
//
//             if *p_word == *word && max < freq {
//                 max = freq;
//                 best_word = p_word.to_owned();
//                 break 'trained;
//             }
//
//         }
//     }
//
//     best_word
// }
//
// #[cfg(test)]
// mod word_probability_tests {
//     use super::word_probability;
//
//     #[test]
//     fn returns_highest() {
//         let poss = vec!["test".to_owned(), "fest".to_owned(), "rest".to_owned()];
//         let trained = vec![("fest".to_owned(), 2), ("test".to_owned(), 5),
//                             ("rest".to_owned(), 3)];
//         assert_eq!("test", word_probability(&poss, &trained));
//     }
//
//     #[test]
//     fn word_not_found() {
//         let poss = vec!["test".to_owned(), "fest".to_owned(), "rest".to_owned()];
//         let trained = vec![("nest".to_owned(), 5), ("lest".to_owned(), 2),
//                             ("jest".to_owned(), 3)];
//         assert_eq!("-", word_probability(&poss, &trained));
//     }
//
// }
