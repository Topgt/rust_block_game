use wasm_bindgen::prelude::*;

pub mod my_crate;

pub use crate::my_crate::shape::Square;
pub use crate::my_crate::container::ContainerBox;

#[wasm_bindgen]
pub struct Game {
  container: ContainerBox,
  next_square: Square
  // #[wasm_bindgen(skip)]
  // pub value: Vec<u32>,
}

#[wasm_bindgen]
impl Game {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Game {
      container: ContainerBox::new(18, 24),
      next_square: Square::new(None, None)
    }
  }

  pub fn add_square(&mut self) {
    if self.container.current_square == None && !self.container.is_full {
      self.container.add_square(self.next_square);
      self.next_square = Square::new(None, None);
    }
  }

  pub fn container_is_full(&self) -> bool {
    self.container.is_full
  }

  pub fn move_square_left(&mut self) {
    self.container.move_square_left();
  }

  pub fn move_square_right(&mut self) {
    self.container.move_square_right();
  }

  pub fn move_square_down(&mut self) {
    self.container.move_square_down();
  }

  pub fn clockwise_rotate_square(&mut self) {
    self.container.clockwise_rotate_square();
  }

  pub fn counterclockwise_rotate_square(&mut self) {
    self.container.counterclockwise_rotate_square();
  }

  pub fn get_container_value(&mut self) -> Vec<u32> {
    self.container.value.clone()
  }

}






