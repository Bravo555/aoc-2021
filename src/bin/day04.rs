use std::fs;

type Board = Vec<i32>;
const BOARD_SIZE: usize = 5;

fn main() {
    let input = fs::read_to_string("input/day04.txt").unwrap();

    let (numbers, boards_str) = input.split_once("\n\n").unwrap();
    let numbers = numbers.split(',').map(|n| n.parse().unwrap());

    let mut boards = parse_boards(boards_str);

    // contains board and number at the time of winning
    let mut won_last: Option<(Board, i32)> = None;
    for marked in numbers {
        boards = boards
            .into_iter()
            .map(|mut board| {
                mark(&mut board, marked);
                board
            })
            .filter(|board| {
                if is_winning(board) {
                    let score: i32 = board.iter().filter(|&&n| n != -1).sum::<i32>() * marked;

                    if won_last.is_none() {
                        println!("won first score: {}", score);
                    }

                    won_last.replace((board.clone(), marked));
                    false
                } else {
                    true
                }
            })
            .collect();
    }

    let (won_last, marked) = dbg!(won_last.unwrap());
    let won_last = dbg!(won_last);
    let score_last = won_last.iter().filter(|&&n| n != -1).sum::<i32>() * marked;
    println!("won last score: {}", score_last);
}

fn parse_boards(input: &str) -> Vec<Board> {
    input
        .split("\n\n")
        .map(|board_str| {
            board_str
                .split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect()
}

fn mark(board: &mut Board, marked: i32) {
    board
        .iter_mut()
        .filter(|num| **num == marked)
        .for_each(|num| *num = -1);
}

fn is_winning(board: &Board) -> bool {
    let row_winning = board
        .chunks_exact(BOARD_SIZE)
        .map(|row| row.iter().sum::<i32>())
        .any(|row| row == -(BOARD_SIZE as i32));

    let column_winning = (0..BOARD_SIZE)
        .map(|i| board.iter().skip(i).step_by(BOARD_SIZE).sum::<i32>())
        .any(|col| col == -(BOARD_SIZE as i32));

    row_winning || column_winning
}
