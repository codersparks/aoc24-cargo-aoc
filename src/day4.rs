use ndarray::{ Array1, Array2, ArrayView1};
use aoc_generators::processors::board_generator::generate_2d_board_char;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Array2<char> {
    generate_2d_board_char(input)
}


#[aoc(day4, part1)]
fn part_1(board: &Array2<char>) -> usize {
    let word_chars_vec = "XMAS".chars().collect::<Vec<char>>();
    let word_chars_view = Array1::from(word_chars_vec);
    let reversed_word_chars_vec = "SAMX".chars().collect::<Vec<char>>();
    let reversed_word_chars_view = Array1::from(reversed_word_chars_vec);
    check_board_for_word(&word_chars_view, &reversed_word_chars_view, &board)
}

fn check_board_for_word(word_chars: &Array1<char>, word_chars_reversed: &Array1<char>,  board: &Array2<char>) -> usize {
    let row_count = check_rows_for_chars(word_chars, word_chars_reversed, board);
    let col_count = check_cols_for_chars(word_chars, word_chars_reversed, board);
    let diag_count = check_diag_for_chars(word_chars, word_chars_reversed, board);

    row_count + col_count + diag_count
}

fn check_axis_for_chars(word_chars: &Array1<char>, word_chars_reversed: &Array1<char>,  axis: &ArrayView1<char>) -> usize {
    axis.windows(word_chars.len()).into_iter().filter(|window| {

        let result = window.eq(word_chars) || window.eq(word_chars_reversed);
        result
    }).map(|window| {
        window
    }).count()
}

fn check_rows_for_chars(word_chars: &Array1<char>, word_chars_reversed: &Array1<char>,  board: &Array2<char>) -> usize {

    board.rows().into_iter().map(|row| {
        check_axis_for_chars(word_chars, word_chars_reversed, &row)
    }).sum::<usize>()
}

fn check_cols_for_chars(word_chars: &Array1<char>, word_chars_reversed: &Array1<char>,  board: &Array2<char>) -> usize {
    board.columns().into_iter().map(|col| {
        check_axis_for_chars(word_chars, word_chars_reversed, &col)
    }).sum::<usize>()
}

fn check_diag_for_chars(word_chars: &Array1<char>, word_chars_reversed: &Array1<char>,  board: &Array2<char>) -> usize {
    board.windows([word_chars.len(),word_chars.len()]).into_iter().map(|window|{
        let mut sum = 0usize;

        //println!("Window:\n{}", window);
        let mut diag = Vec::with_capacity(window.len());
        for i in 0..word_chars.len() {
            let c = *window.get([i,i]).unwrap();
            diag.push(c);
        }

        if (&diag.as_slice()).eq(&word_chars.as_slice().unwrap()) || (&diag.as_slice()).eq(&word_chars_reversed.as_slice().unwrap()) {
            sum += 1;
        }

        let mut reversed_diag = Vec::with_capacity(window.len());
        for i in 0..word_chars.len() {
            let c = *window.get((i,word_chars.len()-1-i)).unwrap();
            reversed_diag.push(c);
        }

        if (&reversed_diag.as_slice()).eq(&word_chars.as_slice().unwrap()) || (&reversed_diag.as_slice()).eq(&word_chars_reversed.as_slice().unwrap()) {
            sum += 1;
        }
        sum
    }).sum::<usize>()
}

fn check_diag_for_chars_part_2(word_chars: &Array1<char>, word_chars_reversed: &Array1<char>,  board: &Array2<char>) -> usize {
    board.windows([word_chars.len(),word_chars.len()]).into_iter().filter(|window|{
        let mut found_x = true;

        //println!("Window:\n{}", window);
        let mut diag = Vec::with_capacity(window.len());
        for i in 0..word_chars.len() {
            let c = *window.get([i,i]).unwrap();
            diag.push(c);
        }

        found_x &= (&diag.as_slice()).eq(&word_chars.as_slice().unwrap()) || (&diag.as_slice()).eq(&word_chars_reversed.as_slice().unwrap());

        let mut reversed_diag = Vec::with_capacity(window.len());
        for i in 0..word_chars.len() {
            let c = *window.get((i,word_chars.len()-1-i)).unwrap();
            reversed_diag.push(c);
        }

        found_x &= (&reversed_diag.as_slice()).eq(&word_chars.as_slice().unwrap()) || (&reversed_diag.as_slice()).eq(&word_chars_reversed.as_slice().unwrap());

        found_x

    }).count()
}

#[aoc(day4, part2)]
fn part_2(board: &Array2<char>) -> usize {
    let word_chars_vec = "MAS".chars().collect::<Vec<char>>();
    let word_chars_view = Array1::from(word_chars_vec);
    let reversed_word_chars_vec = "SAM".chars().collect::<Vec<char>>();
    let reversed_word_chars_view = Array1::from(reversed_word_chars_vec);
    check_diag_for_chars_part_2(&word_chars_view, &reversed_word_chars_view, &board)
}


#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn test_check_rows_for_chars() {
        let board = input_generator(read_to_string("test_input/day4_rows.txt").unwrap().as_str());
        let result = check_rows_for_chars(&"XMAS".chars().collect(), &"SAMX".chars().collect(), &board);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_check_cols_for_chars() {
        let board = input_generator(read_to_string("test_input/day4_cols.txt").unwrap().as_str());
        let result = check_cols_for_chars(&"XMAS".chars().collect(), &"SAMX".chars().collect(), &board);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_check_diag_for_chars() {
        let board = input_generator(read_to_string("test_input/day4_diag.txt").unwrap().as_str());
        let result = check_diag_for_chars(&"XMAS".chars().collect(), &"SAMX".chars().collect(), &board);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_check_board_for_word() {
        let board = input_generator(read_to_string("test_input/day4.txt").unwrap().as_str());
        let result = check_board_for_word(&"XMAS".chars().collect(), &"SAMX".chars().collect(), &board);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_2() {
        let board = input_generator(read_to_string("test_input/day4.txt").unwrap().as_str());
        let result = part_2(&board);

        assert_eq!(result, 9);
    }

}