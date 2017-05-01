# Statistical Spell Checker 

For this homework, you will design and implement a statistical spelling correction program, correct, based on an idea of Peter Norvig’s (Links to an external site.)Links to an external site.. The objective is to deepen your knowledge of Rust and its module system, and to start thinking about performance.

## The concept
The purpose of correct is to find possible corrections for misspelled words. It consists of two phases: The first phase is a training module, which consumes a corpus of correctly spelled words and counts the number of occurrences of each word. The second phase uses the results of the first to check individual words. Specifically, it checks whether each word is spelled correctly according to the training module and, if not, whether “small edits” can reach a variant that is correctly spelled.
