use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let host = "www.example.com";
  let path = "/";

  let mut stream = TcpStream::connect((host, 80))?;

  let request = format!(
    "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
    path, host
  );

  stream.write_all(request.as_bytes())?;

  let mut response = Vec::new();
  stream.read_to_end(&mut response)?;

  let response = String::from_utf8_lossy(&response);
  println!("{}", response);

  Ok(())
}
