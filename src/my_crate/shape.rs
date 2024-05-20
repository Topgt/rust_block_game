use std::vec;

pub trait BlockTrait {
}
#[derive(Debug)]
pub struct Square {
  pub matrix: Vec<Vec<u8>>,
  pub edge: usize,
  pub value: Vec<u8>,
}

impl Square {
  fn matrix_to_number(&mut self) {
    let mut x = 0;
    while x < self.matrix.len() {
      let mut y = 0;
      let mut decimal = 0;
      while y < self.matrix[x].len() {
        decimal += self.matrix[x][y] << self.matrix[x].len() - y - 1;
        y += 1;
      }
      self.value[x] = decimal;
      x += 1;
    }
  }

  fn number_to_matrix(&mut self) {
    let mut x = 0;
    while x < self.value.len() {
      let mut v: u8 = self.value[x];
      let mut y: usize = self.matrix[x].len() - 1;
      self.matrix[x] = vec![0_u8; self.matrix[x].len()];
      while v > 0 {
        let bit = v % 2;
        self.matrix[x][y] = bit;
        v = v >> 1;
        if y == 0 { break; }
        y -= 1;
      }
      x += 1;
    }
  }

  pub fn new(value: Option<Vec<u8>>, matrix: Option<Vec<Vec<u8>>>) -> Self {
    if let Some(v) = value {
      let mut square = Self {
        value: v.clone(),
        matrix: vec![vec![0; v.len() as usize]; v.len() as usize],
        edge: v.len() as usize,
      };
      square.number_to_matrix();
      square
    } else if let Some(m) = matrix {
      let mut square = Self {
        value: vec![0; m.len() as usize],
        matrix: m.clone(),
        edge: m.len() as usize,
      };
      square.matrix_to_number();
      square
    } else {
      let mut square: Square = Self {
        matrix: vec![
          vec![0, 0, 1, 0],
          vec![0, 0, 1, 0],
          vec![0, 0, 1, 0],
          vec![0, 0, 1, 0],
        ],
        value: vec![0; 4],
        edge: 4,
      };
      square.matrix_to_number();
      square
    }
  }

  pub fn clockwise_rotate(&mut self) {
    let max_index = self.matrix.len() - 1;
    for x in 0..=max_index {
      for y in x..max_index - x {
        (
          self.matrix[x][y],
          self.matrix[y][max_index - x],
          self.matrix[max_index - x][max_index - y],
          self.matrix[max_index - y][x]
        ) = (
          self.matrix[max_index - y][x],
          self.matrix[x][y],
          self.matrix[y][max_index - x],
          self.matrix[max_index - x][max_index - y]
        )
      }
    }
    self.matrix_to_number();
  }
  
  pub fn counterclockwise_rotate(&mut self) {
    let max_index = self.matrix.len() - 1;
    for x in 0..=max_index {
      for y in x..max_index - x {
        (
          self.matrix[max_index - y][x],
          self.matrix[x][y],
          self.matrix[y][max_index - x],
          self.matrix[max_index - x][max_index - y]
        ) = (
          self.matrix[x][y],
          self.matrix[y][max_index - x],
          self.matrix[max_index - x][max_index - y],
          self.matrix[max_index - y][x]
        )
      }
    }
    self.matrix_to_number();
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    print!("+++++++++++++++\n");
    let max_index = self.matrix.len() - 1;
    for x in 0..=max_index {
      println!("{:?}", self.matrix[x])
    }
    print!("--------\n");
    println!("{:?}", self.value);
    print!("+++++++++++++++\n");
  }

}
  
