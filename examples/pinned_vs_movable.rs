#![allow(unused)]

fn pinned_unmovable() {
  println!("Pinned");

  /**
   * Here, you're pinning a reference (&0u8).
   * When you use Pin::new(), it guarantees that the value will not be moved
   * (the reference itself is "pinned"). However, the important
   * thing to note is that you're pinning a reference to a value (&0u8),
   * not the value itself. The value itself is still moveable,
   * but the reference will always point to the same memory location,
   * even if moved.
   */
  let x = std::pin::Pin::new(&0u8);

  println!("ref\t: {:p}", x.as_ref());

  let moved = x.clone();

  println!("moved\t: {:p}", moved.as_ref()); // same address of `x`, not moved memory
}

fn movable() {
  println!("Simple");
  let x = Box::new(0u8);

  println!("ref\t: {:p}", x.as_ref());

  let moved = x.clone();

  println!("moved\t: {:p}", moved.as_ref()); // diff address of `x`, it have been moved memory
}

fn main() {
  movable();

  println!();
  println!();

  pinned_unmovable();
}
