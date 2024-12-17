pub struct DayOutput {
    part1: Option<String>,
    part2: Option<String>,
}

impl DayOutput {
    pub fn new() -> DayOutput {
        DayOutput {
            part1: None,
            part2: None,
        }
    }

    pub fn part1(&mut self, result: String) {
        if self.part1.is_some() {
            panic!()
        }
        self.part1 = Some(result)
    }

    pub fn part2(&mut self, result: String) {
        if self.part2.is_some() {
            panic!()
        }
        self.part2 = Some(result)
    }

    pub fn get_part1(&self) -> &Option<String> {
        &self.part1
    }

    pub fn get_part2(&self) -> &Option<String> {
        &self.part2
    }
}
