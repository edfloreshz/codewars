fn last_digit(str1: &str, str2: &str) -> i32 {
  if str2 == "0" { return 1; }
  let x = str1.chars().last().unwrap().to_digit(10).unwrap();
  let m = str2.chars().fold(0, |a,x| (a*10 + x.to_digit(10).unwrap()) % 4);
  let exp = if m == 0 { 4 } else { m };
  (x.pow(exp) % 10) as i32
}

#[test]
fn returns_expected() {
  assert_eq!(last_digit("4s", "1"), 4);
  assert_eq!(last_digit("4", "2"), 6);
  assert_eq!(last_digit("9", "7"), 9);
  assert_eq!(last_digit("10","10000000000"), 0);
  assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
  assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
  assert_eq!(last_digit("4392440701494836686456480519665-14788731028999471874378928171101097931974091675617124933235833101927627518", "68819615221552997273737174557165657483427362207517952651"), 7);
}