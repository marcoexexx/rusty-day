#![allow(unused)]

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::time::Duration;

#[derive(Clone, Debug)]
struct Cat {
  name: String,
  _marker: PhantomPinned,
}

impl Cat {
  fn new(name: &str) -> Self {
    Self {
      name: String::from(name),
      _marker: PhantomPinned,
    }
  }
}

fn main() {
  // let mut tom = Cat::new("Tome");
  // let mut tom = Box::new(Cat::new("Tome"));
  let mut tom = Box::pin(Cat::new("Tome"));

  let moved = tom.name; // Cant move
}
