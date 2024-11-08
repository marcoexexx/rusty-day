#![allow(unused)]

struct Data {
  data: String,
  location: *const String,
}

impl Data {
  fn new(data: String) -> Self {
    Data {
      data,
      location: std::ptr::null(),
    }
  }

  fn init_self_ref(&mut self) {
    self.location = &self.data;
  }

  fn init_self_ref_pinned(mut self: std::pin::Pin<&mut Self>) {
    self.location = &self.data;
  }

  fn show(&self) {
    unsafe {
      println!("`{1}` from {0:p}", self.location, &*self.location);
    }
  }
}

fn main() {
  // let mut data = Data::new(String::from("hello"));
  let mut data = Box::pin(Data::new(String::from("hello")));

  data.as_mut().init_self_ref_pinned();

  println!("ref\t\t: {:p}", &data.data as *const String);
  println!("self-ref\t: {:p}", data.location);
  data.show();

  println!();

  let moved = data;

  println!("moved\t\t: {:p}", &moved.data as *const String);
  println!("self-ref\t: {:p}", moved.location);
  moved.show();
}
