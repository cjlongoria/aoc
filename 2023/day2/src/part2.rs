use crate::gameconfig::GameConfig;

pub fn solution(data: String) {
    let mut games_powered: Vec<usize> = Vec::new();

    for line in data.lines() {
        let mut largest_game = GameConfig::new(0, 0, 0);
        let first_parse: Vec<&str> = line.split(":").collect();
        let games: Vec<&str> = first_parse.last().unwrap().split(";").collect();
        for game in games {
            let current_game = GameConfig::from_string(game);
            largest_game = largest_game.larger_game(current_game);
        }
        games_powered.push(largest_game.power());
    }
    println!(
        "Day 2 part 2 answer - {}",
        games_powered.iter().sum::<usize>()
    )
}

