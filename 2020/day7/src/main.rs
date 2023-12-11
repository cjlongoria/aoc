use std::collections::HashMap;
use std::fs;
// Used only for part 1
// use std::collections::HashSet;


fn main() {
    let contents: String = fs::read_to_string("data/data.txt")
        .expect("Should have been able to read the file");
    
    let color_map = parse_data(contents);
    
    // let mut color_list: HashSet<&str> = HashSet::new();
    // let count = recurse_colors(&color_map, &mut color_list, "shiny gold");
    let count = recurse_color_counter(&color_map, "shiny gold");
    println!("{}", count);


}

// fn recurse_colors<'a>(hm: &'a HashMap<String, Vec<String>>, hl: &mut HashSet<&'a str>, color: &str) -> usize {
//     if let Some(color_list) =  hm.get(color) {
//         for color in color_list {
//             hl.insert(color);
//             recurse_colors(hm, hl, color);}}
//     hl.len()
// }


fn recurse_color_counter<'a>(hm: &'a HashMap<String, Vec<(usize,String)>>, color: &str) -> usize {
    // I started with this and littered dbg! statements in it, then transformed it to the functional style below
    // let mut counter = 0;
    // if let Some(color_list) =  hm.get(color) {
    //     for (num, bag_color) in color_list {
    //         counter += num + (num * recurse_color_counter(hm, bag_color));
    //         // dbg!(bag_color, num, counter);
    //     }}
    // counter

    hm
    .get(color)
    .and_then(|colors| Some(colors
        .iter()
        .fold(0, |acc, color| acc + (color.0 + (color.0 * recurse_color_counter(hm, &color.1))))))
    .unwrap_or_default()

}

fn parse_data(raw: String) -> HashMap<String, Vec<(usize,String)>>{
    let mut hm: HashMap<String, Vec<(usize,String)>> = HashMap::new();
    for line in raw.lines(){
        let mut first_split = line.split(" bags contain ");
        let key = first_split.next().unwrap().to_string();
        let mut data: Vec<(usize,String)> = first_split.next().unwrap()
            .replace("bags","")
            .replace("bag","")
            .replace(".","")
            .split(",")
            .map(|bag| {
                if bag == "no other " {
                    (0,String::from("none"))
                } else {
                    (bag.trim()[..1].parse().unwrap(), bag[2..].trim().to_string())
                }})
            .collect();
        hm.entry(key).and_modify(|e| e.append(&mut data)).or_insert(data);
    }
    hm
    // The below function flips the keys and values in the hashmap and is used in part 1.
    // invert_hm(hm)
}

// fn invert_hm(hm: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
//     let mut inverted_hm: HashMap<String, Vec<String>> = HashMap::new();
//     for (key, values) in hm {
//         for value in values {
//             inverted_hm.entry(value).and_modify(|e| e.push(key.clone())).or_insert(vec![key.clone()]); 
//         }
//     }
//     inverted_hm
// }
