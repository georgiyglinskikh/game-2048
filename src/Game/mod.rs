mod Field;

use ncurses::*;
use Field::Cell::*;

pub struct Game {
    field: Field::Field,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Game {
            field: Field::Field::new(size),
        }
    }

    pub fn run(&mut self) {
        initscr();

        self.draw();

        while let input = getch() {
            match input as u8 {
                b'q' => break,
                b'w' | b'a' | b's' | b'd' => {
                    self.add_new(rand::random::<u8>() % 4);
                }
                _ => {}
            }

            self.draw();
        }

        endwin();
    }

    fn draw(&self) {
        clear();

        for i in 0..self.field.cells.len() {
            for j in 0..self.field.cells[i].len() {
                mvprintw(
                    (i + 1) as i32,
                    (j * 4 + 1) as i32,
                    format!("{}", self.field.cells[i][j].0).as_str(),
                );
            }
        }

        refresh();
    }

    fn add_new(&mut self, n: u8) {
        let positions = {
            let mut temp_positions: Vec<[usize; 2]> = vec![];

            let mut i = 0;

            while i < n {
                let (x, y) = (
                    rand::random::<usize>() % self.field.cells.len(),
                    rand::random::<usize>() % self.field.cells.len(),
                );

                if self.field.cells[x][y].0 == 0 {
                    temp_positions.push([x, y]);
                    i += 1;
                }
            }

            temp_positions
        };

        for i in positions {
            let power: u8 = rand::random::<u8>() % 3;

            let number = 2.0_f32.powf(power as f32) as CellType;

            self.field.cells[i[0]][i[1]] = Cell(number);
        }
    }
}
