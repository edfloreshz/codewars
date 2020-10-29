mod solutions;

fn main() {
  solutions::digital_root::digital_root(175115);
  let array = vec![1,2,3,4,5,6,7];
  let sorted_array = solutions::binary_representation::sort_binary(array);
  println!("{:?}", sorted_array);
}
