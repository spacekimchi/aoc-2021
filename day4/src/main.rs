use std::io::{self, Read, Write};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Board {
    sum: usize,
    values: Vec<Vec<i32>>,
    marked_zero: bool,
    won: bool,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sum: {} \n values: {:?}\n\n", self.sum, self.values)
    }
}

impl Board {
    fn index(&self, row: usize, col: usize) -> Option<&i32> {
        self.values.get(row).and_then(|row| row.get(col))
    }

    fn mark_board(&mut self, row: usize, col: usize) -> Result<()> {
        let pos = self.values.get_mut(row).and_then(|row| row.get_mut(col));
        if let Some(val) = pos {
            if *val >= 0 {
                self.sum -= *val as usize;
                *val = *val * -1;
            }
        }
        Ok(())
    }

    fn check_row(&self, row: usize) -> bool {
        if let Some(row) = self.values.get(row) {
            for &val in row {
                if val == 0 && !self.marked_zero {
                    return false
                } else if val > 0 {
                    return false
                }
            }
        }
        true
    }

    fn check_col(&self, col: usize) -> bool {
        for i in 1..=5 {
            if let Some(&val) = self.values.get(i).and_then(|row| row.get(col)) {
                if val == 0 && !self.marked_zero {
                    return false
                } else if val > 0 {
                    return false
                }
            }
        }
        true
    }

    fn print_win(&self, val: usize) -> usize {
        self.sum * val
    }
}

struct Position {
    board_number: usize,
    row: usize,
    col: usize,
}

struct Number {
    positions: Vec<Position>
}


fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;


    //part1(&input)?;
    part2(&input)?;


    Ok(())
}

fn build_board(input: &str, boards: &mut Vec<Board>, numbers: &mut HashMap<usize, Number>, vals: &mut Vec<usize>) -> Result<()> {
    let mut cur_row: usize = 0;
    let mut s = input.lines();
    let nums = s.next().unwrap().split(',').map(|v| v.parse().unwrap()).collect::<Vec<usize>>();
    for num in nums {
        vals.push(num);
    }

    for line in s {
        if line.trim().is_empty() {
            boards.push(Board { sum: 0, values: vec![vec![]; 5], marked_zero: false, won: false });
            cur_row = 0;
        } else {
            for (i, num) in line.trim().split_whitespace().map(|v| v.parse().unwrap()).collect::<Vec<u32>>().iter().enumerate() {
                let board_len = boards.len() - 1;
                let mut board = boards.last_mut().unwrap();
                board.sum += *num as usize;
                board.values.get_mut(cur_row as usize).unwrap().push(*num as i32);
                if let Some(number) = numbers.get_mut(&(*num as usize)) {
                    number.positions.push(Position { board_number: board_len, row: cur_row, col: i });
                } else {
                    numbers.insert(*num as usize, Number { positions: vec![Position { board_number: board_len, row: cur_row, col: i }] });
                }
            }
            cur_row += 1;
        }
    }
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut boards: Vec<Board> = Vec::new();
    let mut vals: Vec<usize> = Vec::new();
    let mut numbers: HashMap<usize, Number> = HashMap::new();

    build_board(&input, &mut boards, &mut numbers, &mut vals)?;

    for val in vals {
        let mut break_condition = false;
        if let Some(num) = numbers.get(&val) {
            for position in &num.positions {
                if let Some(board) = boards.get_mut(position.board_number) {
                    board.mark_board(position.row, position.col)?;
                    if board.check_row(position.row) {
                        writeln!(io::stdout(), "i was a winner {}", board.print_win(val))?;
                        break_condition = true;
                        break;
                    } else if board.check_col(position.col) {
                        writeln!(io::stdout(), "i was a winner {}", board.print_win(val))?;
                        break_condition = true;
                        break;
                    }
                }
            }
        }
        if break_condition {
            break;
        }
    }
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut boards: Vec<Board> = Vec::new();
    let mut vals: Vec<usize> = Vec::new();
    let mut numbers: HashMap<usize, Number> = HashMap::new();

    build_board(&input, &mut boards, &mut numbers, &mut vals)?;

    for val in vals {
        if let Some(num) = numbers.get(&val) {
            for position in &num.positions {
                if let Some(board) = boards.get_mut(position.board_number) {
                    if board.won {
                        continue;
                    }
                    board.mark_board(position.row, position.col)?;
                    if board.check_row(position.row) {
                        board.won = true;
                        writeln!(io::stdout(), "row was a winner {}", board.print_win(val))?;
                    } else if board.check_col(position.col) {
                        board.won = true;
                        writeln!(io::stdout(), "col was a winner {}", board.print_win(val))?;
                    }
                }
            }
        }
    }
    Ok(())
}

