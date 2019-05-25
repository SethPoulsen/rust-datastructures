use std::vec::Vec;

type WordSearch = Vec<Vec<char>>;
type WordSearchAnswers = Vec<Option<(usize, usize)>>;

pub fn search_for_words(word_search: &WordSearch, words: &Vec<&str>) -> WordSearchAnswers {
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
    let directions: Vec<(usize, usize)> = vec![(0, 1), (1, 0), (1, 1)];
    for direction in directions {
        let mut word_match = true;
        let mut row_iter = row_index;
        let mut col_iter = col_index;
        for character in word.chars() {
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

#[cfg(test)]
mod test {
    use super::search_for_words;

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
            search_for_words(&small_ws, &small_ws_words),
            small_ws_solution
        );

        let big_ws = vec![
            vec!['o', 'o', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['w', 'a', 'w', 'a', 'a', 'a', 'a', 'a', 't', 'a'],
            vec!['n', 'a', 'a', 'n', 'a', 'a', 'a', 'a', 'h', 'a'],
            vec!['e', 'a', 'a', 'a', 'e', 'a', 'a', 'a', 'r', 'a'],
            vec!['r', 'm', 'e', 'm', 'o', 'r', 'y', 'a', 'e', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 's', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'h', 'd', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'i', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'n', 'p'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'g', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a']
        ];
        let big_ws_words = vec!["ownership", "memory", "threading"];
        let big_ws_solution = vec![Some((0, 1)), Some((4, 1)), Some((1, 8))];

        assert_eq!(
            search_for_words(&big_ws, &big_ws_words),
            big_ws_solution
        );

    }
}
