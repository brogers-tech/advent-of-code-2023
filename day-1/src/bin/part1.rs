use advent_of_code::read_file;

fn main() {
    let iter_data = read_file("input.txt");
    let answer = iter_data
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
        })
        .map(|number_string| {
            if number_string.len() == 1 {
                number_string.parse::<u32>().unwrap_or(0) * 11
            } else {
                let mut chars = number_string.chars();
                format!(
                    "{}{}",
                    chars.next().unwrap_or('0'),
                    chars.last().unwrap_or('0')
                )
                .parse::<u32>()
                .unwrap_or(0)
            }
        })
        .sum::<u32>();

    println!("{}", answer);
}
