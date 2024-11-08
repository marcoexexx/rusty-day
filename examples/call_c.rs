extern "C" {
  fn printf(x: *const u8, a: u8);
}

fn main() {
  unsafe {
    printf("hello %d".as_ptr(), 12u8);
  }
}
