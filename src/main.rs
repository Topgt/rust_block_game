mod my_crate;
use my_crate::shape::Square;
use my_crate::container::ContainerBox;


fn main() {
  let mut container = ContainerBox::new(30, 20);
  let mut square = Square::new(None, None);
  container.add_square(square);


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

  // container.clockwise_rotate_square();
  // container.clockwise_rotate_square();
  // container.counterclockwise_rotate_square();
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
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  container.move_square_down();
  // container.move_square_down();
  // container.move_square_down();
  // container.move_square_down();


  let mut square = Square::new(None, None);
  container.add_square(square);


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
  container.move_square_down();

  container.print();
}