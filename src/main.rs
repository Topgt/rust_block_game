mod my_crate;
use my_crate::shape::Square;
use my_crate::container::Box;


fn main() {
  let mut container = Box::new(18, 24);
  let mut square = Square::new(None, None);
  
  container.add_square(&mut square);
  // container.move_square_down();
  // container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  // container.move_square_down();
  // container.move_square_down();

  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();
  // container.move_square_left();

  // container.move_square_down();
  // container.move_square_down();
  // container.move_square_down();
  // container.move_square_down();
  // container.move_square_right();
  // container.move_square_right();
  // container.move_square_right();
  // container.move_square_right();
  // container.move_square_right();
  // container.move_square_right();
  // container.move_square_right();
  // container.move_square_right();
  // container.move_square_right();
  // container.move_square_right();
  // container.move_square_right();


  // container.clockwise_rotate_square();

  container.counterclockwise_rotate_square();
  // container.counterclockwise_rotate_square();
  // container.counterclockwise_rotate_square();
  container.print();
}





// let mut bl = Square::new();
// bl.print_block();
// bl.clockwise_rotate();
// bl.print_block();
// bl.counterclockwise_rotate();
// bl.print_block();