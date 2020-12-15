pub mod cell;

pub struct Field {
    pub cells: Vec<Vec<cell::Cell>>,
}

impl Field {
    pub fn new(size: usize) -> Self {
        let mut cells_l: Vec<Vec<cell::Cell>> = vec![];

        for i in 0..size {
            cells_l.push(vec![]);
            for _j in 0..size {
                cells_l[i].push(cell::Cell(0));
            }
        }

        Field { cells: cells_l }
    }
}
