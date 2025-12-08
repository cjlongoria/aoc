use std::collections::HashSet;
use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("data/data.txt").expect("Should have been able to read the file");

    let data_vec: Vec<isize> = contents.lines().map(|x| x.parse::<isize>().unwrap()).collect();
    let num = find_invalid_num(&data_vec);
    find_range(&data_vec, num);
    
}

fn find_invalid_num(data_vec: &Vec<isize>) -> isize {
    let mut remove_pointer: usize = 0;

    let mut xmas_buffer: HashSet<isize> = HashSet::new();
    for i in 0..25 {
        xmas_buffer.insert(data_vec[i]);
    }

    for i in 25..data_vec.len() {
        let data = data_vec[i];

        let mut valid: bool = false;
        for element in &xmas_buffer {
            let diff_val = data - *element;
            if xmas_buffer.contains(&diff_val) {
                valid = true;
                break;
            }
        }
        if !valid {
            return data_vec[i];

        }
        xmas_buffer.insert(data_vec[i]);
        xmas_buffer.remove(&data_vec[remove_pointer]);
        remove_pointer += 1;
    }
    panic!();
}

fn find_range(data_vec: &Vec<isize>, magic_num: isize) {
    let mut bot_pointer: usize = 0;
    let mut running_sum: isize = 0;
    let mut i = 0;
    while i < data_vec.len() {
        
        if running_sum < magic_num {
            running_sum += data_vec[i];
            i += 1
        } else {
            running_sum -= data_vec[bot_pointer];
            bot_pointer += 1;
            
        } 

        if running_sum == magic_num {
                let max_val = data_vec[bot_pointer..i].iter().max().unwrap();
                let min_val = data_vec[bot_pointer..i].iter().min().unwrap();
                println!("{}", *max_val + *min_val);
                return 
            }
    }
}
