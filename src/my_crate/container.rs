// use wasm_bindgen::prelude::*;

use super::shape::Square;

struct BoxArea {
  a: usize,
  b: usize,
}

fn get_max_value(n: &usize) -> u32 {
  let mut max_value = 0_u32;
  let mut i = 0_u32;
  while i < *n as u32 {
    max_value += 1 << i;
    i += 1;
  }
  max_value
}

pub struct ContainerBox {
  box_area: BoxArea,
  pub container: Vec<Vec<u8>>, // 容器的矩阵表示
  pub is_full: bool, // 是否满了
  pub value: Vec<u32>, // 每一行的数值
  max_valueItem: u32, // 行允许的最大值
  pub canel_len: usize, // 行方块满了清除了多少行, 每次方块被放置后计算一次
  pub current_square: Option<Square>, // 当在容器内的方块
  current_x: usize, // 当前容器的x坐标
  square_x: usize, // 方块的x坐标
  square_y: isize, // 方块的y坐标
}

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

  pub fn current_matrix_value(&mut self) -> Vec<u32> {
    let mut x = 0;
    let mut value = self.value.clone();
    while x < value.len() {
      let v = &mut value[x];
      if let Some(square) = &self.current_square {
        if x >= self.square_x && x <= self.square_x + square.edge - 1 {
          let t = square.value[x - self.square_x] as u32;
          if self.square_y > 0 { *v = *v | (t << self.square_y);}
          else { *v = *v | (t >> (-self.square_y));}
        }
      }
      x += 1;
    }
    value
  }

  fn scorekeeper(&mut self) {
    let mut canel_len = 0_usize;
    let mut i = self.box_area.a - 1;
    while i > 0 {
      if self.value[i] >= self.max_valueItem {
        canel_len += 1;
        let mut j = i;
        while j > 0 {
          self.value[j] = self.value[j - 1];
          j -= 1;
        }
        comtinue;
      }
      i -= 1;
    }
    let mut next_x = 0;
    while next_x < self.box_area.a {
      if self.value[next_x] >= 1 {
        break;
      }
      next_x += 1;
    }
    self.is_full = next_x == 0;
    self.current_x = next_x;
    self.canel_len = canel_len;
  }

  fn set_value(&mut self) {
    if let Some(square) = &self.current_square {
      let mut square_h = square.edge;
      while square_h > 0 {
        if square.value[square_h - 1] >= 1 {
          break;
        }
        square_h -= 1;
      }
      for i in 0..square_h {
        let mut t = square.value[i] as u32;
        if self.square_y > 0 { t <<= self.square_y;}
        else { t >>= -self.square_y;}
        self.value[self.square_x + i] |= t;
      }
      self.current_square = None;
      self.scorekeeper();
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
      is_full: false,
      current_x: x - 1,
      square_x: 0,
      square_y: 0,
      canel_len: 0,
      value: vec![0_u32; x],
      max_valueItem: get_max_value(&y),
    };
    container
  }

  pub fn add_square(&mut self, square: Square) {
    self.canel_len = 0_usize;
    self.square_x = 0_usize;
    self.square_y = ((self.box_area.b -  square.edge) / 2) as isize;
    self.current_square = Some(square);

    // self.value[self.box_area.a - 1] = 992;
    // self.value[self.box_area.a - 2] = 512;
    // self.value[self.box_area.a - 3] = 512;
    // self.value[self.box_area.a - 4] = 512;
    // self.value[self.box_area.a - 5] = 960;
    // self.current_x = self.current_x - 2;
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
      let mut square_h = square.edge;
      while square_h > 0 {
        if square.value[square_h - 1] >= 1 {
          break;
        }
        square_h -= 1;
      }
      if self.square_x + square_h >= self.box_area.a {
        can_move = false;
      } else if self.square_x + square_h >= self.current_x {
        for i in 0..square_h {
          if self.square_x + square_h - i < self.current_x {
            break;
          }
          let mut c_value = square.value[square_h - 1 - i] as u32;
          if self.square_y > 0 { c_value <<= self.square_y;}
          else { c_value >>= -self.square_y;}
          let sel_value = self.value[self.square_x + square_h - i];
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
