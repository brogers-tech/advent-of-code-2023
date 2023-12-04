use advent_of_code::read_file;

#[derive(Debug)]
struct SerialMatrix {
    matrix: Vec<Vec<char>>,
}

impl SerialMatrix {
    fn new() -> SerialMatrix {
        SerialMatrix { matrix: Vec::new() }
    }

    fn add_row(&mut self, row: String) {
        self.matrix.push(row.chars().collect::<Vec<char>>());
    }

    fn get_top_left(&self, row: usize, col: usize) -> char {
        *self
            .matrix
            .get(row.wrapping_sub(1))
            .and_then(|row_above| row_above.get(col.wrapping_sub(1)))
            .unwrap_or(&'.')
    }

    fn get_above(&self, row: usize, col: usize) -> char {
        *self
            .matrix
            .get(row.wrapping_sub(1))
            .and_then(|row_above| row_above.get(col))
            .unwrap_or(&'.')
    }

    fn get_top_right(&self, row: usize, col: usize) -> char {
        *self
            .matrix
            .get(row.wrapping_sub(1))
            .and_then(|row_above| row_above.get(col + 1))
            .unwrap_or(&'.')
    }

    fn get_right(&self, row: usize, col: usize) -> char {
        *self
            .matrix
            .get(row)
            .and_then(|row_above| row_above.get(col + 1))
            .unwrap_or(&'.')
    }

    fn get_bottom_right(&self, row: usize, col: usize) -> char {
        *self
            .matrix
            .get(row + 1)
            .and_then(|row_above| row_above.get(col + 1))
            .unwrap_or(&'.')
    }

    fn get_below(&self, row: usize, col: usize) -> char {
        *self
            .matrix
            .get(row + 1)
            .and_then(|row_above| row_above.get(col))
            .unwrap_or(&'.')
    }

    fn get_bottom_left(&self, row: usize, col: usize) -> char {
        *self
            .matrix
            .get(row + 1)
            .and_then(|row_above| row_above.get(col.wrapping_sub(1)))
            .unwrap_or(&'.')
    }

    fn get_left(&self, row: usize, col: usize) -> char {
        *self
            .matrix
            .get(row)
            .and_then(|row_above| row_above.get(col.wrapping_sub(1)))
            .unwrap_or(&'.')
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn parse_data(serial_matrix: SerialMatrix) -> Vec<u32> {
    let mut result = Vec::new();
    let data = serial_matrix.matrix.iter();

    for (row_idx, row) in data.enumerate() {
        let mut digit_string = String::new();
        let mut is_serial = false;

        for (col_idx, col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                digit_string.push(*col);

                let neighbors = vec![
                    serial_matrix.get_left(row_idx, col_idx),
                    serial_matrix.get_top_left(row_idx, col_idx),
                    serial_matrix.get_above(row_idx, col_idx),
                    serial_matrix.get_top_right(row_idx, col_idx),
                    serial_matrix.get_right(row_idx, col_idx),
                    serial_matrix.get_bottom_right(row_idx, col_idx),
                    serial_matrix.get_below(row_idx, col_idx),
                    serial_matrix.get_bottom_left(row_idx, col_idx),
                ];

                if !is_serial {
                    is_serial = neighbors.into_iter().any(|c| is_symbol(c));
                }
            } else {
                if is_serial {
                    result.push(digit_string.parse::<u32>().unwrap_or(0));
                }
                digit_string.clear();
                is_serial = false;
            }
        }

        if is_serial {
            result.push(digit_string.parse::<u32>().unwrap_or(0));
        }
    }

    result
}

fn main() {
    let data = read_file("input.txt");
    let sm = SerialMatrix::new();

    let matrix_data = data.fold(sm, |mut sm, line| {
        sm.add_row(line);
        sm
    });

    let solution = parse_data(matrix_data).iter().sum::<u32>();

    println!("{solution}");
}
