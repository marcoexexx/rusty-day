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

  /// Invalid parameter
  /// `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Pin<P>`
  // fn pinned_method(self: Pin<Self>) {
  //   println!("This method must pin");
  // }

  fn pinned_method_ref(self: Pin<&Self>) {
    println!("This method must pin");
  }

  fn pinned_method_mut_ref(self: Pin<&mut Self>) {
    println!("This method must pin");
  }
}

fn main() {
  // let mut tom = Cat::new("Tome");
  // let mut tom = Box::new(Cat::new("Tome"));
  let mut tom = Box::pin(Cat::new("Tome"));

  tom.as_ref().pinned_method_ref();
  tom.as_mut().pinned_method_mut_ref();

  let moved = tom.name; // Cant move
}
