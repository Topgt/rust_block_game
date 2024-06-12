// use wasm_bindgen::prelude::*;

use super::shape::Square;

struct BoxArea {
  a: usize,
  b: usize,
}
// #[wasm_bindgen]
pub struct ContainerBox {
  box_area: BoxArea,
  pub container: Vec<Vec<u8>>,
  pub live: bool,
  pub value: Vec<u32>,
  pub current_square: Option<Square>,
  current_x: usize,
  square_x: usize,
  square_y: isize,
}

// #[wasm_bindgen]
impl ContainerBox {
  #[allow(dead_code)]
  fn number_to_matrix(&mut self) {
    let mut x = 0;
    while x < self.value.len() {
      let mut v: u32 = self.value[x];
      if let Some(square) = &self.current_square {
        if x >= self.square_x && x <= self.square_x + square.edge - 1 {
          let t = square.value[x - self.square_x] as u32;
          if self.square_y > 0 { v = v | (t << self.square_y);}
          else { v = v | (t >> (-self.square_y));}
        }
      }
      let mut y: usize = self.box_area.b - 1;
      self.container[x] = vec![0_u8; self.box_area.b];
      while v > 0 {
        let bit: u8 = (v % 2).try_into().unwrap();
        self.container[x][y] = bit;
        v = v >> 1;
        if y == 0 {break;}
        y -= 1;
      }
      x += 1;
    }
  }

  fn set_value(&mut self) {
    if let Some(square) = &self.current_square {
      let mut x = self.current_x;
      for i in 0..square.edge {
        let mut t = square.value[i] as u32;
        if t == 0 { x += 1; }
        if self.square_y > 0 { t <<= self.square_y;}
        else { t >>= -self.square_y;}
        self.value[self.square_x + i] |= t;
      }
      self.current_x = x;
      self.current_square = None;
    }
  }

  #[allow(dead_code)]
  pub fn print(&mut self) {
    self.number_to_matrix();
    println!("{:?}", self.value);
    println!("+++++");
    for row in self.container.iter() {
      for col in row.iter() {
        print!("{} ", col);
      }
      println!("");
    }
  }

  pub fn new(x:usize, y:usize) -> Self {
    let container = ContainerBox {
      box_area: BoxArea {a: x, b:y},
      container: vec![vec![0; y]; x],
      current_square: None,
      live: true,
      current_x: x - 1,
      square_x: 0,
      square_y: 0,
      value: vec![0_u32; x],
    };
    container
  }

  pub fn add_square(&mut self, square: Square) {
    self.square_x = 0_usize;
    self.square_y = ((self.box_area.b -  square.edge) / 2) as isize;
    self.current_square = Some(square);

    self.value[self.box_area.a - 1] = 16775167;
    self.value[self.box_area.a - 2] = 16775167;
    self.value[self.box_area.a - 3] = 16775167;
    self.current_x = self.current_x - 2;
  }

  #[allow(dead_code)]
  pub fn move_square_left(&mut self) {
    if let Some(square) = &self.current_square {
      let mut can_move: bool = true;
      let max: u32 = 1 << (self.box_area.b - 1);
      for i in 0..square.edge {
        let mut c_value = square.value[i] as u32;
        if self.square_y > 0 { c_value <<= self.square_y;}
        else { c_value >>= -self.square_y;}
        let sel_value = self.value[self.square_x + i];
        if c_value >= max || sel_value & (c_value << 1) > 0 {
          can_move = false;
          break;
        }
      }
      if can_move {
        self.square_y += 1
      }
    }
  }

  #[allow(dead_code)]
  pub fn move_square_right(&mut self) {
    if let Some(square) = &self.current_square {
      let mut can_move = true;
      for i in 0..square.edge {
        let mut c_value = square.value[i] as u32;
        if self.square_y > 0 { c_value <<= self.square_y;}
        else { c_value >>= -self.square_y;}
        let sel_value = self.value[self.square_x + i];
        if (c_value % 2 | 0) == 1 || sel_value & (c_value >> 1) > 0 {
          can_move = false;
          break;
        }
      }
      if can_move {
        self.square_y -= 1
      }
    }
  }

  pub fn move_square_down(&mut self) {
    if let Some(square) = &self.current_square {
      let mut can_move = true;
      if self.square_x + square.edge >= self.box_area.a {
        can_move = false;
      } else if self.square_x + square.edge >= self.current_x {
        for i in 0..square.edge {
          if self.square_x + square.edge - i < self.current_x {
            break;
          }
          let mut c_value = square.value[square.edge - 1 - i] as u32;
          if self.square_y > 0 { c_value <<= self.square_y;}
          else { c_value >>= -self.square_y;}
          let sel_value = self.value[self.square_x + square.edge - i];
          if c_value & sel_value > 0 {
            can_move = false;
          }
        }
      }

      if can_move {
        self.square_x += 1;
      } else {
        self.set_value();
        println!("需要添加下一个方块");
      }
    
    }
  }

  #[allow(dead_code)]
  pub fn clockwise_rotate_square(&mut self) {
    if let Some(square) = &mut self.current_square {
      let max: u32 = 1 << self.box_area.b;
      let s_value = square.value.clone();
      let mut t_square = Square::new(Some(s_value), None);
      let mut before_sate: usize = 0;
      for i in 0..t_square.edge {
        if t_square.value[t_square.edge - 1 - i] != 0 { break; }
        before_sate += 1;
      }
      t_square.clockwise_rotate();
      let mut after_sate: usize = 0;
      for i in 0..t_square.edge {
        if t_square.value[t_square.edge - 1 - i] != 0 { break; }
        after_sate += 1;
      }
      let mut sate= 0;
      if after_sate > before_sate { sate = after_sate - before_sate; }

      let mut can_rotate = true;
      for i in 0..t_square.edge {
        let mut c_value = t_square.value[i] as u32;
        if self.square_y > 0 { c_value <<= self.square_y;}
        else { c_value >>= -self.square_y;}
        let sel_value = self.value[self.square_x + i + sate];   
        if c_value & sel_value > 0 || (c_value % 2 | 0) == 1 || c_value >= max {
          can_rotate = false;
          break;
        }
      }
      if can_rotate {
        square.clockwise_rotate();
      }
    }
  }
  
  #[allow(dead_code)]
  pub fn counterclockwise_rotate_square(&mut self) {
    if let Some(square) = &mut self.current_square {
      let max: u32 = 1 << self.box_area.b;
      let s_value = square.value.clone();
      let mut t_square = Square::new(Some(s_value), None);
      let mut before_sate: usize = 0;
      for i in 0..t_square.edge {
        if t_square.value[t_square.edge - 1 - i] != 0 { break; }
        before_sate += 1;
      }
      t_square.counterclockwise_rotate();
      let mut after_sate: usize = 0;
      for i in 0..t_square.edge {
        if t_square.value[t_square.edge - 1 - i] != 0 { break; }
        after_sate += 1;
      }
      let mut sate= 0;
      if after_sate > before_sate { sate = after_sate - before_sate; }
      let mut can_rotate = true;
      for i in 0..t_square.edge {
        let mut c_value = t_square.value[i] as u32;
        if self.square_y > 0 { c_value <<= self.square_y;}
        else { c_value >>= -self.square_y;}
        let sel_value = self.value[self.square_x + i + sate];
        if c_value & sel_value > 0 || (c_value % 2 | 0) == 1 || c_value >= max {
          can_rotate = false;
          break;
        }
      }
      if can_rotate {
        square.counterclockwise_rotate();
      }
    }
  }
}
