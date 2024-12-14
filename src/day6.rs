use std::clone::Clone;
use std::collections::HashSet;
use std::fmt::{ Display, Formatter};
use aoc_generators::processors::board_generator::generate_2d_board_char;
use ndarray::Array2;


#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Invalid direction: {}", c)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GuardTurn {
    direction: Direction,
    row: usize,
    column: usize,
}

#[derive(Debug, Clone)]
pub struct Guard {
    starting_row: usize,
    starting_column: usize,
    starting_direction: Direction,
    turns: Vec<GuardTurn>,
    travelled_distances: Vec<usize>,
    visited_cells: HashSet<(usize, usize)>,
    left_maze: bool
}

impl Display for Guard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Starting Row: {}, Starting Column: {}, Starting Direction: {:?}, Turns: {:?}", self.starting_row, self.starting_column, self.starting_direction, self.turns)
    }
}

impl Guard {
    pub fn new(starting_row: usize, starting_column: usize, starting_direction: Direction) -> Guard {
        let mut g = Guard {
            starting_row,
            starting_column,
            starting_direction,
            turns: Vec::new(),
            travelled_distances: Vec::new(),
            visited_cells: HashSet::new(),
            left_maze: false
        };

        g.visited_cells.insert((starting_row, starting_column));
        g
    }

    pub fn get_current_state(&self) -> Option<(&Direction, usize, usize)> {

        if self.left_maze {
            None
        } else {
            if self.turns.len() == 0 {
                Some((&self.starting_direction, self.starting_row, self.starting_column))
            } else {
                let last_turn = &self.turns.last().unwrap();
                Some((&last_turn.direction, last_turn.row, last_turn.column))
            }
        }
    }

    pub fn move_to_next_turn(&mut self, board: &Array2<char>) -> Option<()> {
        if let Some((current_direction, row, column)) = self.get_current_state() {

            match current_direction {
                Direction::Up => {
                    let col_view = board.column(column);
                    println!("Column View: {:?}", col_view);
                    let mut i = row;
                    loop {
                        if *col_view.get(i).unwrap() == '#' {
                            let turn = GuardTurn {
                                direction: Direction::Right,
                                row: i + 1,
                                column,
                            };
                            let distance = row - i - 1;
                            self.turns.push(turn);
                            self.travelled_distances.push(distance);
                            return Some(());
                        }
                        self.visited_cells.insert((i, column));

                        if i == 0usize {
                            break;
                        }
                        i -= 1;
                    }
                    // If we get here we've not hit an obstacle so we leave the board
                    self.travelled_distances.push(row);
                }
                Direction::Down => {
                    let col_view = board.column(column);
                    println!("Column View: {:?}", col_view);
                    for i in row..col_view.len() {
                        if *col_view.get(i).unwrap() == '#' {
                            let turn = GuardTurn {
                                direction: Direction::Left,
                                column,
                                row: i -1,
                            };
                            self.turns.push(turn);
                            self.travelled_distances.push(i - 1 - row);
                            return Some(());
                        }
                        self.visited_cells.insert((i, column));
                    }
                    // If we get here we've not hit an obstacle so we leave the board
                    self.travelled_distances.push(col_view.len() - 1 - row);

                }
                Direction::Left => {
                    let row_view = board.row(row);
                    println!("RowView: {:?}", row_view);
                    let mut i = row;
                    loop {
                        if *row_view.get(i).unwrap() == '#' {
                            let turn = GuardTurn {
                                direction: Direction::Up,
                                column: i + 1,
                                row,
                            };
                            let distance = column - i -1;
                            self.turns.push(turn);
                            self.travelled_distances.push(distance);
                            return Some(());
                        }

                        self.visited_cells.insert((row, i));

                        if i == 0usize {
                            break;
                        }
                        i -= 1;
                    }
                    self.travelled_distances.push(column);
                }
                Direction::Right => {
                    let row_view = board.row(row);
                    println!("RowView: {:?}", row_view);
                    for i in column..row_view.len() {
                        if *row_view.get(i).unwrap() == '#' {
                            let turn = GuardTurn {
                                direction: Direction::Down,
                                column: i - 1,
                                row,
                            };
                            let distance = i -1 - column;
                            self.turns.push(turn);
                            self.travelled_distances.push(distance);
                            return Some(());
                        }

                        self.visited_cells.insert((row, i));
                    }
                    self.travelled_distances.push(row_view.len() - 1 - column);
                }
            }
        }

        None


    }

    pub fn patrol(&mut self, board: &Array2<char>) -> usize {
        while self.move_to_next_turn(board).is_some() {
            println!("Distance: {}, Last turn: {:?}", self.travelled_distances.last().unwrap(), self.turns.last().unwrap());
            println!("{:?}", self.visited_cells.len());
        }

        println!("Cells Visited: {:?}, Total: {}", self.visited_cells, self.visited_cells.len());
        self.visited_cells.len()
    }

}

#[aoc_generator(day6)]
fn input_generator_day6(input: &str) -> (Array2<char>, Guard) {

    let board = generate_2d_board_char(input);

    let mut starting_details = None;
    for ((row, column), &cell) in board.indexed_iter() {
        match cell {
            '>' | '<' | '^' | 'v' => {
                starting_details = Some((row as usize, column as usize, cell));
                break;
            }
            _ => (),
        }
    }
    if let Some((row, column, cell)) = starting_details {
        (board, Guard::new(row, column , Direction::from_char(cell)))
    } else {
        panic!("No starting details found");
    }
}

#[aoc(day6, part1)]
fn part1(input: &(Array2<char>, Guard)) -> u32 {
    let (board, guard) = input;
    println!("{:?}", board);

    let mut guard = guard.clone();
    println!("{:?}", guard);

    guard.patrol(&board) as u32

}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn test_patrol() {
        let input = read_to_string("test_input/2024/day6/day6.txt").unwrap();
        let (board, guard) = input_generator_day6(&input);
        println!("{:?}", board);
        let mut guard = guard.clone();
        guard.patrol(&board);

        assert_eq!(41, guard.visited_cells.len());

    }

    #[test]
    fn test_part_1_direction_up_obstruction() {
        let input = read_to_string("test_input/2024/day6/day6_starting_up_obstruction.txt").unwrap();

        let (board, guard) = input_generator_day6(&input);
        println!("{:?}", board);
        let mut guard = guard.clone();
        println!("Guard: {}", guard);

        let result = guard.move_to_next_turn(&board);

        assert!(result.is_some());

        let (current_direction, row, column) = guard.get_current_state().unwrap();
        assert!(matches!(*current_direction, Direction::Right));
        assert_eq!(row, 1);
        assert_eq!(*guard.travelled_distances.last().unwrap(), 2usize);
        assert_eq!(column, guard.starting_column);

    }

    #[test]
    fn test_part_1_direction_up_no_obstruction() {
        let input = read_to_string("test_input/2024/day6/day6_starting_up_no_obstruction.txt").unwrap();

        let (board, guard) = input_generator_day6(&input);
        println!("{:?}", board);
        let mut guard = guard.clone();
        println!("Guard: {}", guard);

        let result = guard.move_to_next_turn(&board);

        assert!(result.is_none());
        assert_eq!(*guard.travelled_distances.last().unwrap(), 4usize);


    }

    #[test]
    fn test_part_1_direction_down_obstruction() {
        let input = read_to_string("test_input/2024/day6/day6_starting_down_obstruction.txt").unwrap();

        let (board, guard) = input_generator_day6(&input);
        println!("{:?}", board);
        let mut guard = guard.clone();
        println!("Guard: {}", guard);

        let result = guard.move_to_next_turn(&board);

        assert!(result.is_some());

        let (current_direction, row, column) = guard.get_current_state().unwrap();
        assert!(matches!(*current_direction, Direction::Left));
        assert_eq!(row, 2);
        assert_eq!(*guard.travelled_distances.last().unwrap(), 0usize);
        assert_eq!(column, guard.starting_column);

    }

    #[test]
    fn test_part_1_direction_down_no_obstruction() {
        let input = read_to_string("test_input/2024/day6/day6_starting_down_no_obstruction.txt").unwrap();

        let (board, guard) = input_generator_day6(&input);
        println!("{:?}", board);
        let mut guard = guard.clone();
        println!("Guard: {}", guard);

        let result = guard.move_to_next_turn(&board);

        assert!(result.is_none());
        assert_eq!(*guard.travelled_distances.last().unwrap(), 2usize);


    }

    #[test]
    fn test_part_1_direction_left_obstruction() {
        let input = read_to_string("test_input/2024/day6/day6_starting_left_obstruction.txt").unwrap();

        let (board, guard) = input_generator_day6(&input);
        println!("{:?}", board);
        let mut guard = guard.clone();
        println!("Guard: {}", guard);

        let result = guard.move_to_next_turn(&board);

        assert!(result.is_some());

        let (current_direction, row, column) = guard.get_current_state().unwrap();
        assert!(matches!(*current_direction, Direction::Up));
        assert_eq!(row, guard.starting_row);
        assert_eq!(*guard.travelled_distances.last().unwrap(), 1usize);
        assert_eq!(column, 2);

    }

    #[test]
    fn test_part_1_direction_left_no_obstruction() {
        let input = read_to_string("test_input/2024/day6/day6_starting_left_no_obstruction.txt").unwrap();

        let (board, guard) = input_generator_day6(&input);
        println!("{:?}", board);
        let mut guard = guard.clone();
        println!("Guard: {}", guard);

        let result = guard.move_to_next_turn(&board);

        assert!(result.is_none());
        assert_eq!(*guard.travelled_distances.last().unwrap(), 4usize);


    }

    #[test]
    fn test_part_1_direction_right_obstruction() {
        let input = read_to_string("test_input/2024/day6/day6_starting_right_obstruction.txt").unwrap();

        let (board, guard) = input_generator_day6(&input);
        println!("{:?}", board);
        let mut guard = guard.clone();
        println!("Guard: {}", guard);

        let result = guard.move_to_next_turn(&board);

        assert!(result.is_some());

        let (current_direction, row, column) = guard.get_current_state().unwrap();
        assert!(matches!(*current_direction, Direction::Down));
        assert_eq!(row, guard.starting_row);
        assert_eq!(*guard.travelled_distances.last().unwrap(), 2usize);
        assert_eq!(column, 2);

    }

    #[test]
    fn test_part_1_direction_right_no_obstruction() {
        let input = read_to_string("test_input/2024/day6/day6_starting_right_no_obstruction.txt").unwrap();

        let (board, guard) = input_generator_day6(&input);
        println!("{:?}", board);
        let mut guard = guard.clone();
        println!("Guard: {}", guard);

        let result = guard.move_to_next_turn(&board);

        assert!(result.is_none());
        assert_eq!(*guard.travelled_distances.last().unwrap(), 0usize);


    }

}