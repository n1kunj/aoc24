#[derive(Debug, Clone)]
pub struct Row<T> {
    pub tiles: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct Map<T> {
    pub rows: Vec<Row<T>>,
}

impl<T: Copy> Map<T> {
    pub fn at(&self, (x, y): (isize, isize)) -> Option<T> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        let row = self.rows.get(y)?;
        row.tiles.get(x).copied()
    }
}

impl<T> Map<T> {
    pub fn at_mut(&mut self, (x, y): (isize, isize)) -> Option<&mut T> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        let row = self.rows.get_mut(y)?;
        row.tiles.get_mut(x)
    }
}
