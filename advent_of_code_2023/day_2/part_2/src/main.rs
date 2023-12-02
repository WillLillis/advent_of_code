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

    fn power(&self) -> u32 {
        let min_red = self.info.iter().map(|info| info.red).max().unwrap_or(0);
        let min_green = self.info.iter().map(|info| info.green).max().unwrap_or(0);
        let min_blue = self.info.iter().map(|info| info.blue).max().unwrap_or(0);

        min_red * min_green * min_blue
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
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file");
    let games = get_games(&input);

    let sum: u32 = games.iter().map(|game| game.power()).sum();

    println!("Sum Power: {sum}");
}
