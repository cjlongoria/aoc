#[derive(Debug)]
struct Point((i32, i32));

struct Coords {
    current: Point,
    tile: Tile,
}
impl Coords {
    fn new(point: Point, point_value: char) -> Self {
        Self {
            current: point,
            tile: Tile::new(point_value),
        }
    }

    fn connections(&self) -> Option<(Point, Point)> {
        match self.tile {
            Tile::UpDown => None,
            Tile::LeftRight => None,
            Tile::UpRight => None,
            Tile::UpLeft => None,
            Tile::RightDown => None,
            Tile::LeftDown => None,
            Tile::Start => None,
            Tile::Ground => None,
        }
    }

    fn tile_type(&self) -> &Tile {
        &self.tile
    }

    fn pos(&self) -> &Point {
        &self.current
    }
}

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
            '.' => Tile::Start,
            'S' => Tile::Ground,
            _ => panic!(),
        }
    }
}
pub fn solve(data: &str) -> () {
    for (line_num, line) in data.lines().enumerate() {
        for (pos, ch) in line.chars().enumerate() {
            let point = Point((line_num.try_into().unwrap(), pos.try_into().unwrap()));
            let coords = Coords::new(point, ch);
            let connections = coords.connections();
            println!("Map position is: {:?}", coords.pos());
            if let Some(connections) = connections {
                println!("connection points: {:?}", &connections);
            }
        }
    }
}

