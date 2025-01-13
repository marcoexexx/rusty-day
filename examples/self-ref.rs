use core::marker::PhantomPinned;
use core::pin::Pin;

struct SelfRef {
  data: String,
  point: *const String,
  _marker: PhantomPinned,
}

impl SelfRef {
  fn new(data: String) -> Self {
    SelfRef {
      data,
      point: std::ptr::null(),
      _marker: PhantomPinned,
    }
  }

  fn _set_data(&mut self, data: String) {
    self.data = data
  }

  fn _init(&mut self) {
    self.point = &self.data as *const String;
  }

  fn set_data(self: Pin<&mut Self>, data: String) {
    unsafe { self.get_unchecked_mut().data = data }
  }

  fn init(self: Pin<&mut Self>) {
    let ptr = &self.data as *const String;
    unsafe {
      self.get_unchecked_mut().point = ptr;
    }
  }

  fn locate(&self) {
    unsafe { println!("{:p} -> {}", self.point, *self.point) }
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut data = Box::pin(SelfRef::new("hello".into()));

  data.as_mut().init();
  data.locate();

  data.as_mut().set_data("world".into());
  data.locate();

  Ok(())
}
