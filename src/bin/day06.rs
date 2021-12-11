use std::fs;

fn main() {
    let input = fs::read_to_string("input/day06.txt").unwrap();

    let days = 256;
    let fish: Vec<u8> = input
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();

    // group the amount of fish with a certain timer.
    // on the 0th index we keep fish with timer = 0
    // on the 1th index we keep fish with timer = 1
    // etc.
    let mut fish_map: Vec<u64> = vec![0; 9];

    for f in &fish {
        fish_map[*f as usize] += 1;
    }

    for _ in 0..days {
        let new_fish = fish_map[0];
        fish_map.remove(0);
        fish_map.push(0);
        fish_map[8] += new_fish;
        fish_map[6] += new_fish;
    }

    println!("{}", fish_map.into_iter().sum::<u64>());
}
