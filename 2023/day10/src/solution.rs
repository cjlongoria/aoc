#[derive(Debug, Clone)]
struct Point((usize, usize));

#[derive(PartialEq, Debug)]
enum Tile {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    RightDown,
    LeftDown,
    Start,
    Ground,
}
impl Tile {
    fn new(ch: char) -> Self {
        match ch {
            '|' => Tile::UpDown,
            '-' => Tile::LeftRight,
            'L' => Tile::UpRight,
            'J' => Tile::UpLeft,
            '7' => Tile::RightDown,
            'F' => Tile::LeftDown,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!(),
        }
    }
}

struct Graph {
    data: Vec<Vec<char>>,
    start: Point,
}

impl Graph {
    fn new(raw_data: &str) -> Self {
        let data: Vec<Vec<char>> = raw_data
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let start = data
            .iter()
            .enumerate()
            .find_map(|(col_idx, column)| {
                column
                    .iter()
                    .enumerate()
                    .filter(|(_, &val)| Tile::new(val) == Tile::Start)
                    .map(|(row_idx, _)| Point((col_idx, row_idx)))
                    .next()
            })
            .unwrap();

        Self { data, start }
    }

    fn get_value(&self, point: Point) -> Tile {
        let (row, column) = point.0;
        let value = self.data[row][column];
        Tile::new(value)
    }

    fn get_start(&self) -> &Point {
        &self.start
    }
}

pub fn solve(data: &str) -> () {
    let graph = Graph::new(data);
    dbg!(&graph.get_start());
    dbg!(graph.get_value(graph.get_start().clone()));
}

