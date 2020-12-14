mod Cell;

struct Field {
    cells: Vec<Vec<Cell>>,
}

impl Field {
    pub fn new(size: usize) -> Self {
        let mut cells_l: Vec<Vec<Cell>> = vec![];

        for i in 0..size {
            cells_l.push(vec![]);
            for _j in 0..size {
                cells_l[i].push(Cell(0));
            }
        }

        Field { cells: cells_l }
    }
}
