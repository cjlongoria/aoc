use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("data/data.txt").expect("Should have been able to read the file");

    let data: Vec<&str> = contents.lines().collect();
    let _arrive_time: usize = data[0].parse().unwrap();
    let (residues, modulii) = data[1]
        .split(',')
        .enumerate()
        .filter_map(|(i, x)| match x.parse::<isize>() {
            Ok(x) => Some((x - i as isize, x)),
            Err(_) => None,
        })
        .unzip();
    let ans = chinese_remainder(residues, modulii);
    println!("Answer - {:?}", ans);
}
#[allow(dead_code)]
fn part_1(time: usize, ids: Vec<usize>) -> usize {
    let mut bus_id: usize = 0;
    let mut min_time: usize = usize::MAX;
    for id in ids {
        if id - (time % id) < min_time {
            bus_id = id;
            min_time = id - (time % id);
        }
    }
    bus_id * min_time
}

#[allow(dead_code)]
fn find_min_x(idxlist: Vec<(usize, usize)>) -> isize {
    println!("{:?}", idxlist);
    let total_product = idxlist.iter().fold(1, |acc, &(_, x)| acc * x);
    dbg!(&total_product);
    let common_num = idxlist.iter().fold(0, |acc, &(i, x)| {
        let divided_out_prod = total_product / x;
        dbg!(&divided_out_prod);
        // let rem = if i == 0 { 1 } else { i };
        let rem = i;
        let inv = inv(divided_out_prod as isize, x as isize);
        // let inv = if inv == 0 { 1 } else { inv };
        let mini_sum = rem * divided_out_prod * inv as usize;
        dbg!(&mini_sum);
        acc + mini_sum
    });
    dbg!(&common_num);
    dbg!(common_num % total_product);
    0
}
fn egcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn inv(x: isize, n: isize) -> isize {
    let (_g, x, _) = egcd(x, n);
    (x % n + n) % n
}

fn chinese_remainder(residues: Vec<isize>, modulii: Vec<isize>) -> isize {
    let prod = modulii.iter().product::<isize>();

    let mut sum = 0;

    for (residue, modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * inv(p, modulus) * p
    }

    sum % prod
}
