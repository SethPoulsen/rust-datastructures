use std::vec::Vec;

type WordSearch = Vec<Vec<char>>;
type WordSearchAnswers = Vec<Option<(usize, usize)>>;

pub fn search_words_iterative(word_search: &WordSearch, words: &Vec<&str>) -> WordSearchAnswers {
    let mut answers: WordSearchAnswers = Vec::new();
    for word in words {
        let mut answer = None;
        for row_index in 0..word_search.len() {
            let row = &word_search[row_index];
            for col_index in 0..row.len() {
                answer = match search_for_word_iterative(word_search, word, row_index, col_index) {
                    true => Some((row_index, col_index)),
                    false => None
                };
                if answer != None {
                    break;
                }
            }
            if answer != None {
                    break;
            }
        }
        answers.push(answer);
    }
    answers
}

// Q's to answer:
// can I use i32 to index vec/array? No. cast with "n as usize" be aware, this cast won't tell you about overflow!
// what happens when I access vec/array out of bounds?: Panic

fn search_for_word_iterative(word_search: &WordSearch, word: &str, row_index: usize, col_index: usize) -> bool {
    // this abstraction may be a bit inaccessible for people just begining to learn algorithms! how can I make it simpler?
    let directions: Vec<(usize, usize)> = vec![(1,1),(0, 1), (1, 0), (1, 1)];
    let mut row_iter = row_index;
    let mut col_iter = col_index;
    for direction in directions {
        let mut word_match = true;
        for character in word.chars() {
            // TODO is there a more elegant way to do the iteration than all this i32 <-> usize casting?
            if row_iter >= word_search.len() ||
               col_iter >= word_search.len() ||
               word_search[row_iter][col_iter] != character {
                word_match = false;
                break;
            }
            row_iter += direction.0;
            col_iter += direction.1;
        }

        if word_match {
            return true;
        }
    }
    false
}

pub fn search_words_recursive(word_search: &WordSearch, words: &Vec<&str>) -> WordSearchAnswers {
    unimplemented!()
}

#[cfg(test)]
mod test {
    // use super::WordSearch;
    use super::search_words_iterative;
    use super::search_words_recursive;

    #[test]
    fn basics() {
        let small_ws = vec![
            vec!['c', 'p', 'l'],
            vec!['c', 'a', 'i'],
            vec!['c', 'w', 't']
        ];
        let small_ws_words = vec!["cat", "it", "paw", "dog"];
        let small_ws_solution = vec![Some((0, 0)), Some((1, 2)), Some((0, 1)), None];

        assert_eq!(
            search_words_iterative(&small_ws, &small_ws_words),
            small_ws_solution
        );

        assert_eq!(
            search_words_recursive(&small_ws, &small_ws_words),
            small_ws_solution
        );
    }
}
