use advent_of_code::read_file;
use std::collections::HashSet;

fn main() {
    let data = read_file("input.txt");

    let win_data = data
        .map(|line| line.split(": ").last().unwrap().to_string())
        .map(|card_data| {
            card_data
                .split(" | ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|card_data| {
            let mut i_card_data = card_data.iter();
            let winning_numbers = HashSet::<u32>::from_iter(
                i_card_data
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<u32>().unwrap_or(0)),
            );
            let playing_numbers = HashSet::<u32>::from_iter(
                i_card_data
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<u32>().unwrap_or(0)),
            );

            (winning_numbers, playing_numbers)
        })
        .map(|(winning_numbers, playing_numbers)| {
            winning_numbers
                .intersection(&playing_numbers)
                .collect::<Vec<&u32>>()
                .len()
        })
        .collect::<Vec<usize>>();

    let mut results = vec![1; win_data.len()];

    for (idx, wins) in win_data.iter().enumerate() {
        for i in idx + 1..=idx + wins {
            results[i] += results[idx];
        }
    }

    let results: usize = results.iter().sum();
    println!("{results}");
}
