mod solutions;

fn main() {
  let mut array = vec![0,1,4,2,3,4,5,16,6,7,8];
  assert_eq!(vec![0,1,2,4,4,8,16,3,5,6,7], *solutions::binary_representation::sort_binary(&mut array));
}
