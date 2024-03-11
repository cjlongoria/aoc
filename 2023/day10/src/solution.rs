#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
            '7' => Tile::LeftDown,
            'F' => Tile::RightDown,
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

    fn get_value(&self, point: &Point) -> Tile {
        let (row, column) = point.0;
        let value = self.data[row][column];
        Tile::new(value)
    }

    fn get_start(&self) -> &Point {
        &self.start
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = vec![];
        let (row, column) = point.0;
        let max_length = self.data[0].len() - 1;
        let max_depth = self.data.len() - 1;

        if column != 0 {
            let potential_neighbor = Point((row, column - 1));
            match self.get_value(&potential_neighbor) {
                Tile::UpRight => neighbors.push(potential_neighbor),
                Tile::LeftRight => neighbors.push(potential_neighbor),
                Tile::RightDown => neighbors.push(potential_neighbor),
                _ => (),
            }
        }

        if column != max_length {
            let potential_neighbor = Point((row, column + 1));
            match self.get_value(&potential_neighbor) {
                Tile::LeftDown => neighbors.push(potential_neighbor),
                Tile::LeftRight => neighbors.push(potential_neighbor),
                Tile::UpLeft => neighbors.push(potential_neighbor),
                _ => (),
            }
        }

        if row != 0 {
            let potential_neighbor = Point((row - 1, column));
            match self.get_value(&potential_neighbor) {
                Tile::LeftDown => neighbors.push(potential_neighbor),
                Tile::UpDown => neighbors.push(potential_neighbor),
                Tile::RightDown => neighbors.push(potential_neighbor),
                _ => (),
            }
        }

        if row != max_depth {
            let potential_neighbor = Point((row + 1, column));
            match self.get_value(&potential_neighbor) {
                Tile::UpDown => neighbors.push(potential_neighbor),
                Tile::UpLeft => neighbors.push(potential_neighbor),
                Tile::UpRight => neighbors.push(potential_neighbor),
                _ => (),
            }
        }
        neighbors
    }

    fn bfs(&self) -> std::collections::HashSet<Point> {}
    fn dfs(&self) -> std::collections::HashSet<Point> {}
}

pub fn solve(data: &str) -> () {
    let graph = Graph::new(data);
    let start_point = graph.get_start();
    let neighbors = graph.get_neighbors(start_point);
    dbg!(neighbors);
}

