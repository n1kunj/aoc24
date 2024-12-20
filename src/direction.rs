#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn go((x, y): (isize, isize), d: Direction) -> (isize, isize) {
        Direction::go_n((x, y), d, 1)
    }

    pub fn go_n((x, y): (isize, isize), d: Direction, n: isize) -> (isize, isize) {
        match d {
            Direction::Up => (x, y - n),
            Direction::Right => (x + n, y),
            Direction::Down => (x, y + n),
            Direction::Left => (x - n, y),
        }
    }

    pub fn turn_right(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn turn_left(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub fn turn_around(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];
