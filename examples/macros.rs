macro_rules! log {
  ($level:ident, $msg:ident) => {
    println!("[{}] ->> {}", stringify!($level), $msg.replace("_", "-"));
  };
}

fn main() {
  let hello_hello = "heloo_ww";
  log!(INFO, hello_hello);
}
