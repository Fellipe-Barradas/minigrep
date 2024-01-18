
use minigrep::commands::find_word;

mod commun;

use commun::{setup, teardown};

#[test]
fn test_find_word(){
  setup();
  let input = vec!["teste".to_string()];
  let file_path = "test.txt".to_string();
  let result = find_word(&input, &file_path);
  assert!(result.is_ok());
  teardown();
}

#[test]
fn test_find_multiple_words(){
  setup();
  let input = vec!["hello".to_string(), "world".to_string()];
  let file_path = "test.txt".to_string();
  let result = find_word(&input, &file_path);
  assert!(result.is_ok());
  teardown();
}

#[test]
fn test_invalid_path(){
  setup();
  let input = vec!["teste".to_string()];
  let file_path = "tester.txt".to_string();
  let result = find_word(&input, &file_path);
  assert!(result.is_err());
  teardown();
}