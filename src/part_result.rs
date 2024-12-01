pub struct PartResult {
    part1: Option<i64>,
    part2: Option<i64>,
}

impl PartResult {
    pub fn new() -> PartResult {
        PartResult {
            part1: None,
            part2: None,
        }
    }

    pub fn part1(&mut self, result: i64) -> () {
        if self.part1.is_some() {
            panic!()
        }
        self.part1 = Some(result)
    }

    pub fn part2(&mut self, result: i64) -> () {
        if self.part2.is_some() {
            panic!()
        }
        self.part2 = Some(result)
    }

    pub fn get_part1(&self) -> Option<i64> {
        self.part1
    }

    pub fn get_part2(&self) -> Option<i64> {
        self.part2
    }
}
