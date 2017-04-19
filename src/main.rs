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

        /*let split_line = input.split_whitespace();

        let mut corrected = String::new();
        let mut possibilities = vec![];
        
        for word in split_line {
            possibilities = known(&edits1(word), &training_words);
            corrected = best_word(possibilities, training_words)

            println!("{}, {}", word, corrected);
        }*/

    }

}


fn read_words<R: Read>(reader: R) -> Vec<String> {
    //! reads words from stdin and outputs a vector containing the words
    //! from reader (delimited by whitespace)

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


fn counter(words: &Vec<String>) -> HashMap<String, usize> {

    //! takes in a vector of words creates a hashmap with the frequencies
    //! of the words from the vector

    let mut frequencies: HashMap<String, usize> = HashMap::new();

    for w in words {

        // fixed this hopefully?
        let count: usize = *frequencies.entry(w.to_string()).or_insert(0);
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

fn split(word: &str) -> Vec<(String, String)> {

    let mut splits: Vec<(String, String)> = vec![];
    let length = word.len();

    for i in 1..length {
        splits.push((word[0..i].to_owned(), word[i..length].to_owned()));
    }

    splits

}

#[cfg(test)]
mod test_split {

    use super::split;

    #[test]
    fn blank_test() {

        let word = "";

        let expected: Vec<(String, String)> = vec![];
        let actual = split(word);

        // lengths should be equal
        assert_eq!(actual.len(), expected.len());

        // the elements should be equal
        assert_eq!(actual, expected);

    }

    #[test]
    fn simple_test() {

        let word = "test";

        let mut expected: Vec<(String, String)> = vec![];
        expected.push(("t".to_string(), "est".to_string()));
        expected.push(("te".to_string(), "st".to_string()));
        expected.push(("tes".to_string(), "t".to_string()));
        let actual = split(word);

        assert_eq!(actual, expected);

    }

}


fn delete(splits: &Vec<(String, String)>) -> Vec<String> {

    let mut deletes:Vec<String> = vec![];

    for &(ref L, ref R) in splits {

        let mut new_word: String = L.to_string().clone();
        let mut first = true;
        for c in R.chars() {

            if first == true {
                first = false;
                continue
            }

            new_word += &c.to_string();
        }

        deletes.push(new_word);

    }

    deletes

}

#[cfg(test)]
mod delete_tests {

    use super::delete;

    #[test]
    fn delete_single_test() {

        let mut input: Vec<(String, String)> = vec![];
        input.push(("te".to_string(), "st".to_string()));
        let actual: Vec<String> = delete(&input);
        let mut expected: Vec<String> = vec![];
        expected.push("tet".to_string());

        assert_eq!(actual, expected);

    }
    
    #[test]
    fn delete_front_back() {

        let mut input: Vec<(String, String)> = vec![];
        input.push(("tes".to_string(), "t".to_string()));
        let actual: Vec<String> = delete(&input);
        let mut expected: Vec<String> = vec![];
        expected.push("tes".to_string());

    }

    #[test]
    fn delete_multiple() {

        let mut input: Vec<(String, String)> = vec![];
        input.push(("te".to_string(), "st".to_string()));
        input.push(("tes".to_string(), "t".to_string()));
        let actual: Vec<String> = delete(&input);
        let mut expected: Vec<String> = vec![];
        expected.push("tet".to_string());
        expected.push("tes".to_string());

        assert_eq!(actual, expected);

    }

}


fn edits1(word: &str) -> Vec<String> {

    let mut edits1_words: Vec<(String, String)> = vec![];

    let letters = "abcdefghijklmnopqrstuvwxyz";

    // gets all the splitted words
    let mut splits: Vec<(String, String)> = vec![];
    //for w in words {
    splits.append(&mut split(word));
    //}

    let mut possibles: Vec<String> = vec![];

    possibles.append(&mut delete(&splits));
    // let mut transposes: Vec<String> = vec![];
    // let mut replaces: Vec<String> = vec![];
    // let mut inserts: Vec<String> = vec![];


    possibles
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


fn known(edits: &Vec<String>, trained_words: &Vec<String>) -> Vec<String> {

    let mut known_words: Vec<String> = vec![];

    for e in edits {
        if trained_words.contains(e) {
            known_words.push((*e).to_owned());
        }
    }

    known_words

}

#[cfg(test)]
mod known_tests {
    use super::known;

    #[test]
    fn no_words_found() {
        let mut edits = vec![];
        let mut trained_words = vec![];
        let mut expected: Vec<String>  = vec![];
        edits.push("test".to_owned());
        edits.push("tesst".to_owned());
        edits.push("rest".to_owned());
        edits.push("tet".to_owned());

        assert_eq!(expected, known(&edits, &trained_words))
    }

    #[test]
    fn filters_known() {
        let mut edits = vec![];
        let mut trained_words = vec![];
        let mut expected = vec![];
        edits.push("test".to_owned());
        edits.push("tesst".to_owned());
        edits.push("rest".to_owned());
        edits.push("tet".to_owned());
        trained_words.push("test".to_owned());
        trained_words.push("rest".to_owned());
        expected.push("test".to_owned());
        expected.push("rest".to_owned());

        assert_eq!(expected, known(&edits, &trained_words))
    }
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







fn best_word(possible_words: &Vec<String>,
                trained_words: &HashMap<String, usize>) -> String {
    //! Given a vector of possible words, return the word that has the highest
    //! probability based on the training set. Return "-" if no possible words
    //! are found in the training set.

    let mut best_word = String::from("-");
    let mut max = 0;

    for p_word in possible_words {
        match (*trained_words).get(p_word) {
            Some(freq) => {
                if *freq > max {
                    max = *freq;
                    best_word = p_word.to_owned();
                }
            },
            _ => continue,
        }
    }
    best_word
}

#[cfg(test)]
mod best_word_tests {
    use super::best_word;
    use std::collections::HashMap;

    #[test]
    fn returns_highest() {
        let poss = vec!["test".to_owned(), "fest".to_owned(), "rest".to_owned()];
        let mut trained = HashMap::new();
        trained.insert("fest".to_owned(), 2);
        trained.insert("test".to_owned(), 5);
        trained.insert("rest".to_owned(), 3);
        assert_eq!("test", best_word(&poss, &trained));
    }

    #[test]
    fn word_not_found() {
        let poss = vec!["test".to_owned(), "fest".to_owned(), "rest".to_owned()];
        let mut trained = HashMap::new();
        trained.insert("nest".to_owned(), 5);
        trained.insert("lest".to_owned(), 2);
        trained.insert("jest".to_owned(), 3);
        assert_eq!("-", best_word(&poss, &trained));
    }
}
