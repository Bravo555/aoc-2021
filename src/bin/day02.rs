use std::fs;

fn main() {
    let input = fs::read_to_string("input/day02.txt").unwrap();
    let steps = input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut slice| {
            (
                slice.next().unwrap(),
                slice.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold((0, 0, 0), |(acc_x, acc_y, acc_aim), (dir, amt)| match dir {
            "forward" => (acc_x + amt, acc_y + amt * acc_aim, acc_aim),
            "down" => (acc_x, acc_y, acc_aim + amt),
            "up" => (acc_x, acc_y, acc_aim - amt),
            _ => panic!("bad command"),
        });

    println!("{:#?} {:?}", steps, steps.0 * steps.1);
}
