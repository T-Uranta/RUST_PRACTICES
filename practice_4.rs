fn main() {
	let p:f64 = 1000.00;
	let r:f64 = 1.00;
	let t:f64 = 2.00;

	//simple interest
	let a = p * (1.00 + (r * t / 100.00));
	let si = a - p;

	println!("Simple interest is {}", si);
}