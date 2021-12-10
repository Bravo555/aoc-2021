use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("input/day01.txt")
        .unwrap()
        .lines()
        .flat_map(|line| line.parse().ok())
        .collect();

    let increased = zad2(&input);

    println!("increased: {}", increased);
}

fn zad1(mut input: impl Iterator<Item = i32>) -> i32 {
    let mut increased = 0;
    let mut prev = input.next().unwrap();
    for elev in input {
        if elev > prev {
            increased += 1;
        }
        prev = elev;
    }

    increased
}

fn zad2(input: &[i32]) -> i32 {
    let s = input.windows(3).map(|window| window.iter().sum());
    zad1(s)
}
