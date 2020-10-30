pub fn sort_binary(input: &mut Vec<i32>) -> &mut Vec<i32> {
	input.sort();
	input.sort_by(|a, b| a.count_ones().cmp(&b.count_ones()));
	input
}