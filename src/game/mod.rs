mod field;
mod utils;

use ncurses::*;

pub struct Game {
    field: field::Field,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Game {
            field: field::Field::new(size),
        }
    }

    pub fn run(&mut self) {
        initscr();

        self.draw();

        loop {
            let ch = getch() as u8;
            match ch {
                b'q' => break,
                b'w' | b'a' | b's' | b'd' => {
                    self.field.add_new(rand::random::<u8>() % 4);
                    self.field.swipe(utils::Direction::from_ch(ch))
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
}
