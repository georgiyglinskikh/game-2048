mod game;

fn main() {
    println!("Enter size of field: ");

    let mut size_str_buf = String::new();
    std::io::stdin()
        .read_line(&mut size_str_buf)
        .expect("Enter a number");

    let size: usize = size_str_buf.trim().parse().expect("Size should be a number");

    let mut game = game::Game::new(size);

    game.run();
}
