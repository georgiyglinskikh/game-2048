use super::utils::Direction;

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

    pub fn add_new(&mut self, n: u8) {
        use rand::random;

        let positions = {
            let mut temp_positions: Vec<[usize; 2]> = vec![];

            let mut i = 0;

            while i < n {
                let (x, y) = (
                    random::<usize>() % self.cells.len(),
                    random::<usize>() % self.cells.len(),
                );

                if self.cells[x][y].0 == 0 {
                    temp_positions.push([x, y]);
                    i += 1;
                }
            }

            temp_positions
        };

        for i in positions {
            let power: u8 = random::<u8>() % 3;

            let number = 2.0_f32.powf(power as f32) as cell::CellType;

            self.cells[i[0]][i[1]] = cell::Cell(number);
        }
    }

    pub fn swipe(&mut self, dir: Direction) {

    }
}
