struct GameConfig {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameConfig {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }

    fn possible(&self, other: GameConfig) -> bool {
        if other.red <= self.red && other.green <= self.green && other.blue <= self.blue {
            true
        } else {
            false
        }
    }

    fn from_string(results: &str) -> Self {
        let cubes: Vec<&str> = results.split(",").collect();
        let mut game = GameConfig::new(0, 0, 0);
        for cube in cubes {
            let cube_parsed: Vec<&str> = cube.trim().split(" ").collect();
            let count: usize = cube_parsed.first().unwrap().parse().unwrap();
            let color: &str = cube_parsed.last().unwrap();
            match color {
                "blue" => game.blue = count,
                "red" => game.red = count,
                "green" => game.green = count,
                _ => panic!(),
            }
        }
        game
    }
}

pub fn solution(data: String) {
    let gc = GameConfig::new(12, 13, 14);
    let mut possible_games: Vec<usize> = Vec::new();

    for line in data.lines() {
        let first_parse: Vec<&str> = line.split(":").collect();
        let game_id_unparsed = first_parse.first().unwrap();
        let game_id = game_id_unparsed
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut possible_flag: bool = true;
        let games: Vec<&str> = first_parse.last().unwrap().split(";").collect();
        for game in games {
            let current_game = GameConfig::from_string(game);
            if !&gc.possible(current_game) {
                possible_flag = false;
            }
        }
        if possible_flag {
            possible_games.push(game_id);
        }
    }
    println!(
        "Day 2 part 1 answer - {}",
        possible_games.iter().sum::<usize>()
    )
}

