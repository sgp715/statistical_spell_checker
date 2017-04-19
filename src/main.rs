/**
 * statistical_spell_checker
 *
 * This program is a statistical spelling correction program. This program
 * reads words from stdin and offers spelling corrections based on words found
 * in the training file. If no correction can be found, the program returns
 * "-". The implementation of this program is based on the implementation found
 * in http://norvig.com/spell-correct.html
 *
 * Usage: cargo run <training_file>
 *
 */



use std::env;
use std::io::{BufRead, BufReader, Read, stdin};
use std::fs::File;
use std::collections::HashMap;


fn main() {

    // read the training corpus from a file
    let training_filename = env::args().nth(1).expect("Usage: cargo run <training_file>");
    let file = File::open(training_filename).expect("Could not read file");
    let training_words = read_words(file);
    let frequencies = counter(&training_words);

    //println!("Model trained!");
    //println!("Enter words to be corrected (ctrl+C to quit):");


    let mut lines = BufReader::new(stdin()).lines();

    while let Some(Ok(word)) = lines.next() {
        let split_line = word.split_whitespace();

        for word in split_line {
            let cleaned = clean_word(word); // remove special characters

            // Add words that are 1 edit away
            let mut possibilities = vec![];
            let e1_poss = edits1(&cleaned);
            possibilities.append(&mut known(&e1_poss, &training_words));

            // Add words that are 2 edits away
            let e2_poss = edits2(&e1_poss);
            possibilities.append(&mut known(&e2_poss, &training_words));
            let corrected = best_word(&possibilities, &frequencies);

            println!("{}, {}", cleaned, corrected);
        }
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
    use std::io::Cursor;
    use super::read_words;

    #[test]
    fn reads() {
        assert_read(&["hello".to_owned(), "world".to_owned()], "hello world");
    }

    #[test]
    fn handles_whitespace() {
        assert_read(&["hello".to_owned(), "world".to_owned()],
                    "\thello\n \tworld\n");
    }

    fn assert_read(expected: &[String], input: &str) {
        let mock_read = Cursor::new(input);
        let words = read_words(mock_read);
        assert_eq!(expected.to_owned(), words);
    }
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

/// Strips numbers and special characters from the beginning and end
/// of the input string.
fn clean_word(word: &str) -> &str {
    word.trim_matches(|c: char| c == ',' || c == '.' || c == '!'
                      || c == '?' || c == '\'' || c == '\"'
                      || c == '(' || c == ')'  || c == '-'
                      || c == ':' || c == ';')
}

#[cfg(test)]
mod clean_word_tests {
    use super::clean_word;

    #[test]
    fn test_period() {
        let expected = "word";
        let output = clean_word("word.");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_single_quotes() {
        let expected = "word";
        let output = clean_word("\'word\'");
        assert_eq!(expected, output);
    }

    #[test]
    fn test_double_quotes() {
        let expected = "word";
        let output = clean_word("\"word\"");
        assert_eq!(expected, output);
    }
}

fn delete(splits: &Vec<(String, String)>) -> Vec<String> {

    let mut deletes:Vec<String> = vec![];

    for &(ref left, ref right) in splits {

        let mut new_word: String = left.to_string().clone();
        let mut first = true;
        for c in right.chars() {

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

        assert_eq!(actual, expected);

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


fn replace(splits: &Vec<(String, String)>) -> Vec<String> {

    let mut replaces:Vec<String> = vec![];
    let letters = "abcdefghijklmnopqrstuvwxyz";

    for &(ref left, ref right) in splits {

        for l in letters.chars() {

            let mut new_word: String = left.to_string().clone();
            let mut first = true;
            for c in right.chars() {

                if first == true {

                    new_word += &l.to_string();
                    first = false;
                    continue
                }

                new_word += &c.to_string();

            }

            replaces.push(new_word);

        }

    }

    replaces

}

#[cfg(test)]
mod replace_tests {

    use super::replace;

    #[test]
    fn replace_single_test() {

        let mut input: Vec<(String, String)> = vec![];
        input.push(("te".to_string(), "st".to_string()));
        let actual: Vec<String> = replace(&input);
        let mut expected: Vec<String> = vec![];
        expected.push("teat".to_string());

        assert_eq!(actual[0], expected[0]);

    }

}

fn insert(splits: &Vec<(String, String)>) -> Vec<String> {

    let mut inserts:Vec<String> = vec![];
    let letters = "abcdefghijklmnopqrstuvwxyz";

    for &(ref left, ref right) in splits {

        for l in letters.chars() {

            let mut new_word: String = left.to_string().clone();
            let mut first = true;
            for c in right.chars() {

                if first == true {

                    new_word += &l.to_string();
                    first = false;
                }

                new_word += &c.to_string();

            }

            inserts.push(new_word);

        }

    }

    inserts

}

#[cfg(test)]
mod insert_tests {

    use super::insert;

    #[test]
    fn insert_single_test() {

        let mut input: Vec<(String, String)> = vec![];
        input.push(("te".to_string(), "t".to_string()));
        let actual: Vec<String> = insert(&input);
        let mut expected: Vec<String> = vec![];
        expected.push("teat".to_string());

        assert_eq!(actual[0], expected[0]);

    }

}

fn transpose(splits: &Vec<(String, String)>) -> Vec<String> {

    let mut transposes:Vec<String> = vec![];

    for &(ref left, ref right) in splits {

        let mut new_word: String = left.to_string().clone();

        if right.len() >= 2{
            
            let mut first = true;
            let mut second = false;
            let mut new_right1: String = "".to_string();
            let mut new_right0: String = "".to_string();
            let mut new_right_rest: String = "".to_string();
            for c in right.chars(){
                if first == true {
                    new_right0 += &c.to_string();
                    first = false;
                    second = true;
                    continue
                }
                if second == true{
                    new_right1 += &c.to_string();
                    second = false;
                    continue
                }
                new_right_rest += &c.to_string();
            }
            new_word = new_word +&new_right1+&new_right0+&new_right_rest;
            transposes.push(new_word);
        }
        else{
            for c in right.chars(){
                new_word += &c.to_string();
            }
            transposes.push(new_word);
        }
    }

    transposes

}

#[cfg(test)]
mod transposes_tests {

    use super::transpose;

    #[test]
    fn transposes_single_test() {

        let mut input: Vec<(String, String)> = vec![];
        input.push(("te".to_string(), "st".to_string()));
        let actual: Vec<String> = transpose(&input);
        let mut expected: Vec<String> = vec![];
        expected.push("tets".to_string());

        assert_eq!(actual[0], expected[0]);

    }
    #[test]
    fn transposes_empty_test() {

        let mut input: Vec<(String, String)> = vec![];
        input.push(("te".to_string(), "t".to_string()));
        let actual: Vec<String> = transpose(&input);
        let mut expected: Vec<String> = vec![];
        expected.push("tet".to_string());

        assert_eq!(actual[0], expected[0]);

    }

}



fn edits1(word: &str) -> Vec<String> {

    // gets all the splitted words
    let mut splits: Vec<(String, String)> = vec![];
    splits.append(&mut split(word));

    let mut possibles: Vec<String> = vec![];

    possibles.append(&mut delete(&splits));
    possibles.append(&mut transpose(&splits));
    possibles.append(&mut replace(&splits));
    possibles.append(&mut insert(&splits));


    possibles
}

fn edits2(words: &Vec<String>) -> Vec<String> {

    let mut possibles: Vec<String> = vec![];

    for w in words {
        possibles.append(&mut edits1(w));
    }

    possibles
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
        let trained_words = vec![];
        let expected: Vec<String>  = vec![];
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
