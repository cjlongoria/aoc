use std::fs;
use std::collections::HashSet;


fn main() {
    let contents: String = fs::read_to_string("data/data.txt")
        .expect("Should have been able to read the file");

    println!("{}", part2(contents));

}

fn part2(contents: String) -> usize {
    let groups = contents.split("\r\n\r\n");
    let mut total_count =  0;
    for group in groups {
        let mut group_iter = group.lines();
        let mut question_hs: HashSet<char> = group_iter.next().unwrap().chars().collect();
        for person in group.lines() {
            question_hs.retain(|&k| person.chars().any(|x| x==k));
            // if we eliminated all the elements at this point then there isn't a consensus.
            if question_hs.is_empty(){break}
        }
        total_count += question_hs.len();
    }
    total_count
}

// ChatGPT's more functional style
// fn part2(contents: String) -> usize {
//     contents
//         .split("\r\n\r\n")
//         .map(|group| {
//             let mut group_iter = group.lines();
//             let question_hs: HashSet<char> = group_iter.next().unwrap().chars().collect();
//             group_iter
//                 .fold(question_hs, |acc, person| {
//                     acc.intersection(&person.chars().collect::<HashSet<_>>())
//                         .copied()
//                         .collect()
//                 })
//                 .len()
//         })
//         .sum()
// }
