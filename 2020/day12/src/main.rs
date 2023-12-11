use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("data/data.txt").expect("Should have been able to read the file");

    let data: Vec<&str> = contents.lines().collect();
    let hm = where_am_i(&data);
    solve_it(hm);
}

fn where_am_i(data: &Vec<&str>) -> HashMap<char, usize> {
    let mut dir_hm: HashMap<char, usize> = HashMap::new();
    //let mut dir_q: VecDeque<char> = VecDeque::from(['E', 'S', 'W', 'N']);
    let mut dir_q: VecDeque<usize> = VecDeque::from([1,10,0,0]);
    for e in data {
        let dir: char = e.chars().next().unwrap();
        let num: usize = e[1..].parse().unwrap();

        /* Part 1 match is below
        match dir {
            'N' => {
                dir_hm
                    .entry('N')
                    .and_modify(|count| *count += num)
                    .or_insert(num);
            }
            'E' => {
                dir_hm
                    .entry('E')
                    .and_modify(|count| *count += num)
                    .or_insert(num);
            }
            'S' => {
                dir_hm
                    .entry('S')
                    .and_modify(|count| *count += num)
                    .or_insert(num);
            }
            'W' => {
                dir_hm
                    .entry('W')
                    .and_modify(|count| *count += num)
                    .or_insert(num);
            }
            'F' => {
                dir_hm
                    .entry(*dir_q.front().unwrap())
                    .and_modify(|count| *count += num)
                    .or_insert(num);
            }
            'L' => dir_q.rotate_right(num/90),       
            'R' => dir_q.rotate_left(num/90),
            _ => (),
        };
        */
        match dir {
            'N' => dir_q[0] += num,
            'E' => dir_q[1] += num,
            'S' => dir_q[2] += num,
            'W' => dir_q[3] += num,
            'F' => to_the_waypoint(&mut dir_hm, &dir_q, num), 
            'L' => dir_q.rotate_left(num/90),       
            'R' => dir_q.rotate_right(num/90),
            _ => (),
        };
    }
    dir_hm
}

fn solve_it(mut hm: HashMap<char, usize>) {
    let lat: i32 =
        i32::abs(*hm.entry('N').or_default() as i32 - *hm.entry('S').or_default() as i32);
    let long: i32 =
        i32::abs(*hm.entry('W').or_default() as i32 - *hm.entry('E').or_default() as i32);
    println!("Part 1 answer - {}", lat + long);
}

fn to_the_waypoint(hm: &mut HashMap<char, usize>, dir_q: &VecDeque<usize>, times: usize) {
    hm.entry('N').and_modify(|e| *e+= dir_q[0]*times).or_insert(dir_q[0]*times);
    hm.entry('E').and_modify(|e| *e+= dir_q[1]*times).or_insert(dir_q[1]*times);
    hm.entry('S').and_modify(|e| *e+= dir_q[2]*times).or_insert(dir_q[2]*times);
    hm.entry('W').and_modify(|e| *e+= dir_q[3]*times).or_insert(dir_q[3]*times);
}
