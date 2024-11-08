fn main() {
  let vec = (0..3).collect::<Vec<i32>>();
  let mut iter = vec.iter().peekable();

  iter.next();
  iter.next();

  println!("{:?}", iter);
}
