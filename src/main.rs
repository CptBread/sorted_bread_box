use sorted_bread_box::*;

fn main() {
	let mut sorted = SortedVecMap::<i32, i32>::default();
	sorted.sorted_insert(12, 42);
	sorted.sorted_insert(11, 44);
	sorted.sorted_insert(123, 44);
	println!("{:?}", sorted);
}
