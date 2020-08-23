// //https://www.codewars.com/kata/513e08acc600c94f01000001

pub fn rgb(r: i32, g: i32, b: i32) -> String {
  format!("{}{}{}", convert(r), convert(g), convert(b)).to_uppercase()
}

fn convert(n: i32) -> String {
  if n < 0 { format!("00") }
  else if n > 255 { format!("FF") }
  else { format!("{}{}", to_hex(n as u8 / 16), to_hex(n as u8 % 16)) }
}

fn to_hex(n: u8) -> String {
  if n > 9 { format!("{}", (87 + n) as char) }
  else { format!("{}", n) }
}

macro_rules! compare {
  ( $got : expr, $expected : expr ) => {
    if $got != $expected { panic!("Got: {}\nExpected: {}\n", $got, $expected); }
  };
}

#[cfg(test)]
mod sample_tests {
  use self::super::*;

  #[test]
  fn tests() {
    compare!(rgb(0, 0, 0), "000000");
    compare!(rgb(1, 2, 3), "010203");
    compare!(rgb(255, 255, 255), "FFFFFF");
    compare!(rgb(254, 253, 252), "FEFDFC");
    compare!(rgb(-20, 275, 125), "00FF7D");
  }
}
