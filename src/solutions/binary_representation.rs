pub fn sort_binary(input: Vec<i32>) -> Vec<i32> {
	let mut map = input.iter().map(|num| {
		let ones = format!("{:b}", num).chars().filter(|bin| *bin == '1').collect::<String>();
		(num, ones.len())
	}).collect::<Vec<(&i32, usize)>>();
	map.sort_by(|a, b| a.1.cmp(&b.1));
	let map: Vec<i32> = map.iter().map(|x| *x.0).collect::<Vec<i32>>();
	map
}