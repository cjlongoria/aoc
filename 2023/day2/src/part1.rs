use crate::gameconfig::GameConfig;

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

