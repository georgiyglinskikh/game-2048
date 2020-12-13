use ncurses::*;

type CellType = u8;

// ./Game/Field/Cell.rs
struct Cell(CellType);

// ./Game/Field/mod.rs
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

// ./Game/mod.rs
struct Game {
    field: Field,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Game {
            field: Field::new(size),
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

// ./main.rs
fn main() {
    println!("Enter size of field: ");

    let mut size_str_buf = String::new();
    std::io::stdin()
        .read_line(&mut size_str_buf)
        .expect("Enter a number");

    let size: usize = size_str_buf.trim().parse().expect("Size should be a number");

    let mut game = Game::new(size);

    game.run();
}
