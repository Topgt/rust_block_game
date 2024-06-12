struct Game {
  name: String,
  data: Vec<u8>,
}

fn main() {
  let game = Game {
    name: "Block Game".to_string(),
    data: vec![1, 2, 3, 4, 5],
  };

  println!("Game name: {}", game.name);
}