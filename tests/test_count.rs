mod commun;

use commun::{setup, teardown};
use minigrep::commands::count;

#[test]
fn test_count_word(){
  setup();
  let input = String::from("hello");
  let file_path = "test.txt".to_string();
  let result = count(&input, &file_path);
  println!("{:?}", result);
  teardown();
}

#[test]
fn test_invalid_path(){
  setup();
  let input = String::from("hello");
  let file_path = "tester.txt".to_string();
  let result = count(&input, &file_path);
  assert!(result.is_err());
  teardown();
}