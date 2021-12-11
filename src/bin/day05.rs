use std::fs;

type Line = (Point, Point);
type Point = (i32, i32);
type Board = Vec<u8>;

const BOARD_SIDE: usize = 1000;

fn main() {
    let input = fs::read_to_string("input/day05.txt").unwrap();

    let lines: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_once(" -> ")
                .map(|(p1, p2)| {
                    (
                        p1.split_once(",")
                            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                            .unwrap(),
                        p2.split_once(",")
                            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                            .unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();

    let mut board: Board = vec![0; BOARD_SIDE * BOARD_SIDE];

    for line in &lines {
        draw_line(&mut board, line, true);
    }

    let overlap_num = board.iter().filter(|point| **point >= 2).count();
    println!("{}", overlap_num);
}

fn draw_line(board: &mut Board, line: &Line, diag: bool) {
    let &((x1, y1), (x2, y2)) = line;
    if !diag && x1 != x2 && y1 != y2 {
        return;
    }

    let (len_x, len_y) = (x2 - x1, y2 - y1);

    let (mut x, mut y) = (x1, y1);
    let final_point = (x2, y2);
    loop {
        let idx = y * BOARD_SIDE as i32 + x;
        board[idx as usize] += 1;

        if (x, y) == final_point {
            break;
        }

        x += len_x.signum();
        y += len_y.signum();
    }
}
