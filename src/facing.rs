#[derive(Copy, Clone, Debug)]
pub(crate) enum Facing {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Facing {
    pub(crate) fn go((x, y): (isize, isize), d: Facing) -> (isize, isize) {
        match d {
            Facing::N => (x, y - 1),
            Facing::NE => (x + 1, y - 1),
            Facing::E => (x + 1, y),
            Facing::SE => (x + 1, y + 1),
            Facing::S => (x, y + 1),
            Facing::SW => (x - 1, y + 1),
            Facing::W => (x - 1, y),
            Facing::NW => (x - 1, y - 1),
        }
    }
}

pub(crate) const FACINGS: &[Facing] = &[
    Facing::N,
    Facing::NE,
    Facing::E,
    Facing::SE,
    Facing::S,
    Facing::SW,
    Facing::W,
    Facing::NW,
];
