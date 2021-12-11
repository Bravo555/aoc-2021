use std::fs;

fn main() {
    let input = fs::read_to_string("input/day07.txt").unwrap();

    let crabs: Vec<u16> = input.split(',').map(|n| n.parse().unwrap()).collect();

    dbg!(crabs.len());
    dbg!(crabs.iter().min());
    dbg!(crabs.iter().max());

    let sum_part_1 = |crab: u16, pos: u16| (crab as i64 - pos as i64).abs();
    let sum_part_2 = |crab: u16, pos: u16| {
        let diff = sum_part_1(crab, pos);
        (diff * (diff + 1)) / 2
    };

    let least_cost: i64 = (*crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap())
        .map(|pos| crabs.iter().map(|&crab| sum_part_2(crab, pos)).sum())
        .min()
        .unwrap();

    println!("cost: {}", least_cost);
}
