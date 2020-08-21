use std::collections::HashMap;

fn count_duplicates(text: &str) -> u32 {
  let mut count = 0;
  let chars: Vec<char> = text.to_lowercase().chars().collect();
  let mut hasmap = HashMap::new();
  for char in chars.iter() {
    *hasmap.entry(char).or_insert(0) += 1;
  }
  for (_, v) in hasmap {
    if v > 1 {
      count += 1
    }
  }
  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_abcde() {
    assert_eq!(count_duplicates("abcde"), 0);
  }

  #[test]
  fn test_abcdea() {
    assert_eq!(count_duplicates("abcdea"), 1);
  }

  #[test]
  fn test_indivisibility() {
    assert_eq!(count_duplicates("indivisibility"), 1);
  }
}
