pub struct GameConfig {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameConfig {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }

    pub fn possible(&self, other: GameConfig) -> bool {
        if other.red <= self.red && other.green <= self.green && other.blue <= self.blue {
            true
        } else {
            false
        }
    }

    pub fn larger_game(&self, other: GameConfig) -> Self {
        GameConfig {
            red: if self.red >= other.red {
                self.red
            } else {
                other.red
            },
            green: if self.green >= other.green {
                self.green
            } else {
                other.green
            },
            blue: if self.blue >= other.blue {
                self.blue
            } else {
                other.blue
            },
        }
    }

    pub fn from_string(results: &str) -> Self {
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

    pub fn power(&self) -> usize {
        self.red * self.blue * self.green
    }
}

