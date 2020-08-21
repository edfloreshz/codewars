fn solution(word: &str, ending: &str) -> bool {
  let ending_size = ending.len();
  let word_size = word.len();
  if word_size >= ending_size {
    if &word.to_string()[word_size-ending_size..] != ending {
      return false
    }
  } else {
    return false
  }
  true
}

#[test]
fn returns_expected() {
  assert_eq!(true, solution("abc", "c"));
  assert_eq!(false, solution("strawberry", "banana"));
}