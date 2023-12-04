use advent_of_code::read_file;
use std::collections::HashSet;

fn main() {
    let data = read_file("input.txt");

    let solution: u32 = data
        .map(|line| line.split(": ").last().unwrap().to_string())
        .map(|card_data| {
            card_data
                .split(" | ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|card_data| {
            let mut i_card_data = card_data.iter();
            let mut winning_numbers = HashSet::<u32>::from_iter(
                i_card_data
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<u32>().unwrap_or(0)),
            );
            let playing_numbers = i_card_data
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap_or(0));

            let mut score = 0;

            for number in playing_numbers {
                if winning_numbers.contains(&number) {
                    if score == 0 {
                        score = 1;
                    } else {
                        score *= 2;
                    }

                    winning_numbers.remove(&number);
                }
            }

            score
        })
        .sum();

    println!("{solution:?}");
}
