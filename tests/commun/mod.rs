use std::{fs::File, io::{BufWriter, Write}};

pub fn setup() {
  let mut bw = BufWriter::new(File::create("test.txt").unwrap());
  bw.write_all(b"hello world").unwrap();
}

pub fn teardown() {
  std::fs::remove_file("test.txt").unwrap();
}
