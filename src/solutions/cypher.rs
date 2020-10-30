pub struct Cipher {
  alphabet: Vec<char>,
  alternative: Vec<char>
}

impl Cipher {
  pub fn new(map1: &str, map2: &str) -> Cipher {
    Cipher {
      alphabet: map1.chars().collect(),
      alternative: map2.chars().collect()
    }
  }
  
  pub fn encode(&self, string: &str) -> String {
    string.chars().map(|c| {
      if self.alphabet.contains(&c) {
        *self.alternative.get(self.alphabet.iter().position(|&r| r == c).unwrap()).unwrap()
      } else { c }
    }).collect()
  }
  
  pub fn decode(&self, string: &str) -> String {
    string.chars().map(|c| {
      if self.alternative.contains(&c) {
        *self.alphabet.get(self.alternative.iter().position(|&r| r == c).unwrap()).unwrap()
      } else { c }
    }).collect()
  }
}