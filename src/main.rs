mod solutions;

fn main() {
  // solutions::digital_root::digital_root(175115);
  // let array = vec![1,1,1,4,2,3,4,5,6,7];
  // let sorted_array = solutions::binary_representation::sort_binary(array);
  // println!("{:?}", sorted_array);
  let a = solutions::cypher::Cipher::new("asdfghj", "hgfdssa");
  println!("{}", a.encode("as%a"));
  println!("{}", a.decode("hgs"));
}
