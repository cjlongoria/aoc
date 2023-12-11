// use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

fn main() {
    et contents: String =
        fs::read_to_string("data/data.txt").expect("Should have been able to read the file");

    let mut data_vec: Vec<usize> = contents.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    data_vec.sort();
    
    // gen_hm(&data_vec);

    let answer = gen_paths(&mut data_vec);
    println!("{}", answer);
}

fn gen_paths(data: &mut Vec<usize>) -> usize {
    let mut hm: HashMap<usize, usize> = HashMap::new();
    data.reverse();
    data.push(0);
    let mut iter = data.iter();
    let seed_value: usize = *iter.next().unwrap();
    hm.insert(seed_value, 1);
    
    for e in iter {
        let mut sub_count: usize = 0;
        for i in e+1..e+4 {
            sub_count += *hm.entry(i).or_default();
        }
        hm.insert(*e, sub_count);
    }
    *hm.get(&0).unwrap()
}


// fn gen_hm(data: &Vec<usize>) {
//     let mut hm: HashMap<usize, usize> = HashMap::from([(3,1)]);
//     let mut previous_joltage: usize = 0;
    
//     for e in data {
//         let difference = e - previous_joltage;
//         hm.entry(difference).and_modify(|difference: &mut usize| *difference += 1).or_insert(1);
//         previous_joltage = *e;
//     }
//     dbg!(&hm);
// }
