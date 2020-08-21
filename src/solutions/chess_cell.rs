// https://www.codewars.com/kata/5894134c8afa3618c9000146/train/rust

pub fn chessboard_cell_color(x: &str, y: &str) -> bool {
  let (xx,xy) = (x.as_bytes()[0], x.as_bytes()[1]);
  let (yx,yy) = (y.as_bytes()[0], y.as_bytes()[1]);
  (xx + yx + xy + yy) % 2 == 0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_tests() {
    assert_eq!(chessboard_cell_color("A1", "C3"), true);
    assert_eq!(chessboard_cell_color("A1", "H3"), false);
    assert_eq!(chessboard_cell_color("A1", "A2"), false);
    assert_eq!(chessboard_cell_color("A1", "C1"), true);
    assert_eq!(chessboard_cell_color("A1", "A1"), true);
  }
}