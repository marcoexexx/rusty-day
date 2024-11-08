fn simple_box() {
  let data = Box::new(String::from("hello"));
  println!("{:p}", &(*data) as *const String);

  let _moved_underlying_data = data.clone();

  println!("{:p}", &(*_moved_underlying_data) as *const String);
}

fn pinned() {
  let data = std::pin::Pin::new(String::from("hello"));
  println!("{:p}", data.as_ref());

  // Can't move underlying data
  let _moved_underlying_data = data.clone(); // TODO: change

  println!("{:p}", _moved_underlying_data.as_ref());
}

fn main() {
  simple_box();

  println!();

  pinned();
}
