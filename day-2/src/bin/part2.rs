use advent_of_code::read_file;

#[derive(Debug)]
struct GameState {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn with_red(mut self, red: u32) -> GameState {
        self.red = red;
        self
    }

    fn with_green(mut self, green: u32) -> GameState {
        self.green = green;
        self
    }

    fn with_blue(mut self, blue: u32) -> GameState {
        self.blue = blue;
        self
    }

}


fn main() {
    let data = read_file("input.txt");

    let solution: u32 = data.map(|line|{
        let game_data = line.split(": ").nth(1).unwrap();
        let game_data = game_data.split("; ");
        game_data.map(|data| {
            let data = data.split(", ");
            
            let game_state = data.fold(GameState::new(), |state, game|{
                let mut current_game_data = game.split(" ");
                let count = current_game_data.next().unwrap().parse::<u32>().unwrap();
                let color = current_game_data.next().unwrap();

                let state = match color {
                    "red" => state.with_red(count),
                    "green" => state.with_green(count),
                    "blue" => state.with_blue(count),
                    _ => state
                };

                state
            });

            game_state
        }).collect::<Vec<GameState>>()
    })
    .map(|games| {
        let mut max_red = 1;
        let mut max_green = 1;
        let mut max_blue = 1;

        for game in games {
            max_red = u32::max(max_red, game.red);
            max_green = u32::max(max_green, game.green);
            max_blue = u32::max(max_blue, game.blue);
        }

        max_red * max_green * max_blue
    }).sum();

    println!("{solution}");

}
