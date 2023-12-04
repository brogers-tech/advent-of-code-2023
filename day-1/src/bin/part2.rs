use advent_of_code::read_file;

const NUMBER_WORDS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", 
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn word_to_number(string: &str) -> &str {
    match string {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => string
    }
}

fn main() {
    let iter_data = read_file("input.txt");
    let answer = iter_data
        .map(|line| {
            let first = 
                NUMBER_WORDS
                    .into_iter()
                    .fold((isize::MAX, ""), |(idx, curr), word| {
                        let found_idx = match line.find(word) {
                            Some(f_idx) => f_idx as isize,
                            None => idx as isize,
                        };

                        if found_idx < idx {
                            (found_idx, word_to_number(word))
                        } else {
                            (idx, curr)
                        }
                    }).1;

            let last = 
                NUMBER_WORDS
                    .into_iter()
                    .fold((isize::MIN, ""), |(idx, curr), word| {
                        let found_idx = match line.rfind(word) {
                            Some(f_idx) => f_idx as isize,
                            None => idx as isize,
                        };

                        if found_idx > idx {
                            (found_idx, word_to_number(word))
                        } else {
                            (idx, curr)
                        }
                    }).1;
            format!("{}{}", first, last).parse::<u32>().unwrap_or(0)
        })
        .sum::<u32>();

    println!("{}", answer);
}
