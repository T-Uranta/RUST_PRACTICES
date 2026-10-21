fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.00;
	let t:f64 = 3.00;

	let a = p * (1.00 - r / 100.00).powf(t);
	println!("Value of the TV after 3 years is {}", a);
}