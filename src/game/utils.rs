pub enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn from_ch(ch: u8) -> Self {
        match ch {
            b'w' => Direction::Up,
            b's' => Direction::Down,
            b'a' => Direction::Left,
            b'd' => Direction::Right,
            _ => todo!()
        }
    }
}
