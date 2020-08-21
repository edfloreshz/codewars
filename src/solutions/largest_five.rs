fn largest_five_digit_number(num: &str) -> u32 {
  let mut max: u32 = 0;
  if num.len() > 5 {
    for i in 0..num.len()-4 {
      let value = num[i..i+5].parse::<u32>().unwrap();
      if value > max {max = value}
    }
  }
  max
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic() {
    assert_eq!(largest_five_digit_number(&"1234567890"), 67890);
  }
}
