use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    // zad 1
    let bits = count_bits(lines.iter().map(|l| *l));

    let size = bits.len();
    let gamma: u32 = bits
        .iter()
        .enumerate()
        .filter(|(_, b)| **b > (lines.len() / 2))
        .fold(0, |acc, (i, _)| acc + (1 << i));
    let epsilon = !gamma & ((1 << (size)) - 1);

    println!("gamma: {:b}", gamma);
    println!("epsilon: {:b}", epsilon);
    println!("result: {}", gamma * epsilon);

    // zad 2
    let mut oxygen_iter: Vec<_> = (0..lines.len()).collect();
    let mut co2_iter: Vec<_> = (0..lines.len()).collect();
    let mut mask: Vec<_> = bits
        .iter()
        .rev()
        .map(|i| if *i >= lines.len() / 2 { '1' } else { '0' })
        .collect();
    let mut pos = 0;
    while oxygen_iter.len() > 1 {
        oxygen_iter.retain(|idx| lines[*idx].chars().nth(pos).unwrap() == mask[pos]);
        mask = dbg!(count_bits(
            lines
                .iter()
                .enumerate()
                .filter(|(i, _)| oxygen_iter.contains(i))
                .map(|(_, l)| *l),
        ))
        .iter()
        .rev()
        .map(|bits| {
            if *bits >= (oxygen_iter.len() + 1) / 2 {
                '1'
            } else {
                '0'
            }
        })
        .collect();
        pos += 1;
        dbg!(&oxygen_iter);
        dbg!(&mask);
    }
    pos = 0;
    while co2_iter.len() > 1 {
        co2_iter.retain(|idx| lines[*idx].chars().nth(pos).unwrap() != mask[pos]);
        mask = count_bits(
            lines
                .iter()
                .enumerate()
                .filter(|(i, _)| co2_iter.contains(i))
                .map(|(_, l)| *l),
        )
        .iter()
        .rev()
        .map(|i| {
            if *i >= (co2_iter.len() + 1) / 2 {
                '1'
            } else {
                '0'
            }
        })
        .collect();
        pos += 1;
        dbg!(&co2_iter);
    }
    let oxygen = usize::from_str_radix(lines[oxygen_iter[0]], 2).unwrap();
    let co2 = usize::from_str_radix(lines[co2_iter[0]], 2).unwrap();

    println!("oxygen: {}", oxygen);
    println!("co2: {}", co2);
    println!("result: {}", oxygen * co2);
}

fn count_bits<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<usize> {
    let mut lines = lines.peekable();
    let mut bits = vec![0; lines.peek().expect("no input").len()];
    for line in lines {
        for (i, c) in line.chars().rev().enumerate() {
            if c == '1' {
                bits[i] += 1;
            }
        }
    }

    bits
}
