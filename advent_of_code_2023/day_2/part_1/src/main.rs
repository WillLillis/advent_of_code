use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct GameInfo {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameInfo {
    fn new() -> Self {
        GameInfo {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    info: Vec<GameInfo>,
}

impl Game {
    fn new() -> Self {
        Game {
            id: 0,
            info: vec![],
        }
    }

    fn add_info(&mut self, info: GameInfo) {
        self.info.push(info);
    }

    fn is_possible(&self, max_constraint: &GameInfo) -> bool {
        for info in self.info.iter() {
            if info.red > max_constraint.red
                || info.green > max_constraint.green
                || info.blue > max_constraint.blue
            {
                return false;
            }
        }

        true
    }
}

fn get_games(input: &str) -> Vec<Game> {
    let mut games = vec![];
    let game_reg = Regex::new(r"Game (?P<GameId>\d+):").unwrap();
    let red_reg = Regex::new(r"(?P<Count>\d+ red)").unwrap();
    let green_reg = Regex::new(r"(?P<Count>\d+ green)").unwrap();
    let blue_reg = Regex::new(r"(?P<Count>\d+ blue)").unwrap();

    for line in input.lines() {
        let mut curr_game = Game::new();
        curr_game.id = game_reg
            .captures(line)
            .unwrap()
            .name("GameId")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let infos: Vec<&str> = line.split(';').collect();
        for game in infos {
            let mut curr_info = GameInfo::new();
            curr_info.red = if let Some(caps) = red_reg.captures(game) {
                caps.name("Count")
                    .unwrap()
                    .as_str()
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            } else {
                0
            };
            curr_info.green = if let Some(caps) = green_reg.captures(game) {
                caps.name("Count")
                    .unwrap()
                    .as_str()
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            } else {
                0
            };
            curr_info.blue = if let Some(caps) = blue_reg.captures(game) {
                caps.name("Count")
                    .unwrap()
                    .as_str()
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            } else {
                0
            };
            curr_game.add_info(curr_info);
        }
        games.push(curr_game);
    }

    games
}

fn main() {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    let max_constraints = GameInfo {
        red: MAX_RED,
        green: MAX_GREEN,
        blue: MAX_BLUE,
    };

    let input = std::fs::read_to_string("../input").expect("Failed to read the input file");
    let games = get_games(&input);

    let sum: u32 = games
        .iter()
        .map(|game| {
            if game.is_possible(&max_constraints) {
                game.id
            } else {
                0
            }
        })
        .sum();

    println!("Possible sum: {sum}");
}
