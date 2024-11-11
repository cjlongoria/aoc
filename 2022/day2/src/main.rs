mod model;

fn main() {
    let data = include_str!("../data/data");
    let mut max = 0;
    for line in data.lines() {
        // Part 1
        // let (opponent, me): (model::Choice, model::Choice) = line
        // Part 2
        let (opponent, me): (model::Choice, model::Outcome) = line
            .split_once(" ")
            .map(|string_tuple| {
                (
                    string_tuple.0.parse().unwrap(),
                    string_tuple.1.parse().unwrap(),
                )
            })
            .unwrap();
        // Part 1
        // max += me.match_point(&opponent);
        // Part 2
        max += opponent.predict(&me);
    }
    println!("{max}");
}
