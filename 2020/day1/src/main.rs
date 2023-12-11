use std::fs;

fn main() {
    let contents: Vec<i16> = fs::read_to_string("data/data.txt")
        .expect("Should have been able to read the file")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

 
    // Part 1
    // for index in 0..contents.len() {
    //     for other_index in 0..contents.len() {
    //         if index == other_index {continue};
    //         if (contents[index] + contents[other_index]) == 2020 {
    //             println!("{} and {}", contents[index], contents[other_index]);
    //             return
    //         } 
    //     }
    // }

    //n^3 for the win. If rust couldn't handle this a hashmap would reduce runtime complexity
    for index in 0..contents.len() {
        for other_index in 0..contents.len() {
            for last_index in 0..contents.len() {
                if index == other_index {continue};
                if index == last_index {continue};
                if other_index == last_index {continue};
                if (contents[index] + contents[other_index] + contents[last_index]) == 2020 {
                    println!("{} and {} and {}", contents[index], contents[other_index], contents[last_index]);
                    let answer: u64 = (contents[index] as u64) * (contents[other_index] as u64) * (contents[last_index] as u64);
                    println!("{}", answer);
                    return
                } 
            }
        }
    }
}
