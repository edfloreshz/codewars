fn perimeter(n: u64) -> u64 {
  let mut number = 0;
  for i in 0..n+1 {
    number += fib(i);
  }
  number * 4
}

fn fib(n:u64) -> u64{
  let mut res = 0;
  let mut next_item = 1;
  for _ in 0..=n{
    let tmp = next_item;
    next_item += res;
    res = tmp;
  }
  return res;
}

fn dotest(n: u64, exp: u64) -> () {
  assert_eq!(perimeter(n), exp)
}

#[test]
fn basics_perimeter() {
  dotest(5, 80);
  dotest(7, 216);
  dotest(20, 114624);
  dotest(30, 14098308);
}