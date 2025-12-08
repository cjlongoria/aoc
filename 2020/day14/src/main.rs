use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("data/data.txt").expect("Should have been able to read the file");

    let mut hm: HashMap<isize, isize> = HashMap::new();
    let mut current_mask = 0u64;
    let mut current_floating_bits: Vec<usize> = Vec::new();
    for line in contents.lines() {
        let mut temp = line.split(" = ");
        let inst = temp.next().unwrap();
        let data = temp.next().unwrap();

        // Part 1
        // if inst == "mask" {
        //     data_map = data
        //         .chars()
        //         .enumerate()
        //         .filter_map(|(i, x)| x.is_numeric().then_some((i, x)))
        //         .collect();
        // };
        // if inst != "mask" {
        //     // println!("{}", inst);
        //     let mut raw_binary: Vec<char> = format!("{:036b}", data.parse::<i64>().unwrap())
        //         .chars()
        //         .collect();
        //     for (idx, bit) in &data_map {
        //         raw_binary[*idx] = *bit;
        //     }
        //     let binary_string = raw_binary.iter().collect::<String>();
        //     let value = isize::from_str_radix(binary_string.as_str(), 2).unwrap();
        //     let addr = inst
        //         .chars()
        //         .filter(|&x| x.is_numeric())
        //         .collect::<String>()
        //         .parse()
        //         .unwrap();
        //     hm.insert(addr, value);
        // }

        if inst == "mask" {
            current_floating_bits = data
                .chars()
                .enumerate()
                .filter_map(|(i, x)| x.is_alphabetic().then_some(i))
                .collect();
            let temp_binary_string = data.replace('X', "0");
            current_mask = u64::from_str_radix(&temp_binary_string, 2).unwrap();
        }

        if inst != "mask" {
            let addr = inst
                .chars()
                .filter(|&x| x.is_numeric())
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let bitwise_or = addr | current_mask;
            let mut binary_string: Vec<char> = format!("{:036b}", bitwise_or).chars().collect();

            // Setting the base case to all zeros where the mask has an X
            for idx in &current_floating_bits {
                binary_string[*idx] = '0';
            }

            // Iterate through all the combinations of other Xs in the mask
            let mut binary_array: Vec<char> = binary_string.clone();
            for ps in current_floating_bits.iter().powerset() {
                for idx in ps {
                    binary_array[*idx] = '1';
                }
                let temp_str = binary_array.iter().collect::<String>();
                let val = u64::from_str_radix(temp_str.as_str(), 2).unwrap();

                // After changing the bit values from the current combination print the value
                // dbg!(val);

                // Reset the value back to the base version
                binary_array = binary_string.clone();

                // This needs to be parsed to an int
                // dbg!(data);

                hm.insert(val as isize, data.parse().unwrap());
            }
        }
    }
    let ans: isize = hm.values().copied().sum();
    println!("{ans}");
}
