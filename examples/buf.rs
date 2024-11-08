use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::{error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
  let path = Path::new("examples").join("data.txt");

  let mut reader = Rc::new(RefCell::new(BufReader::new(File::open(&path)?)));

  let mut buf = [0u8; 1024];

  let mut _reader = reader.borrow_mut();
  let mut lines: Vec<String> = *_reader
    .lines()
    .map(Result::unwrap)
    .map(String::from)
    .collect();

  if lines.len() > 6 {
    lines[6] = "Modified".to_string();
  }

  let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

  for line in lines {
    writeln!(file, "{}", line)?;
  }

  Rc::clone(&reader)
    .borrow_mut()
    .read_exact(&mut buf)
    .map(|_| {
      println!("read 1kb: {}", String::from_utf8_lossy(&buf));
    })
    .unwrap_or_else(|err| {
      if err.kind() == io::ErrorKind::UnexpectedEof {
        let bytes = buf.iter().take_while(|&&b| b != 0).count();
        if bytes > 0 {
          println!("full file: {}", String::from_utf8_lossy(&buf[..bytes]));
        }
      }
    });

  Ok(())
}
