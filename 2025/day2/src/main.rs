fn main() {
    let data = "8284583-8497825,7171599589-7171806875,726-1031,109709-251143,1039-2064,650391-673817,674522-857785,53851-79525,8874170-8908147,4197684-4326484,22095-51217,92761-107689,23127451-23279882,4145708930-4145757240,375283-509798,585093-612147,7921-11457,899998-1044449,3-19,35-64,244-657,5514-7852,9292905274-9292965269,287261640-287314275,70-129,86249864-86269107,5441357-5687039,2493-5147,93835572-94041507,277109-336732,74668271-74836119,616692-643777,521461-548256,3131219357-3131417388";

    let res = data.split(",").fold(0, |acc, range| {
        let (start, end) = range.split_once("-").unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();
        acc + (end - start)
    });
    println!("{res}");

    let ranges = data.split(",").map(|r| r.split_once("-").unwrap());
    let mut total: usize = 0;

    for (lower, upper) in ranges {
        let lower_int: usize = lower.parse().unwrap();
        let upper_int: usize = upper.parse().unwrap();
        for id_int in lower_int..=upper_int {
            let id = id_int.to_string();
            // part1(&id, &mut total);
            part2(&id, &mut total);
        }
    }
    println!("{total}");
}

fn part1(id: &str, total: &mut usize) {
    // Skip ids that cannot be split evenly.
    if id.len() % 2 != 0 {
        return;
    }
    let halfway_point = id.len() / 2;
    if id[..halfway_point] == id[halfway_point..] {
        *total += id.parse::<usize>().unwrap();
    }
}

fn part2(id: &str, total: &mut usize) {
    let len = id.len() / 2;
    for i in 0..=len {
        let combos = id.split(&id[0..i]).all(|x| x.is_empty());
        if combos {
            *total += id.parse::<usize>().unwrap();
            return;
        }
    }
}
