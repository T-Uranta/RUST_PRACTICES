fn main() {
	let qty1 = 2;
	let qty2 = 1;
	let qty3 = 3;
	let qty4 = 3;
	let qty5 = 1;

	let amt1 = 450_000.00;
	let amt2 = 1_500_000.00;
	let amt3 = 750_000.00;
	let amt4 = 2_850_000.00;
	let amt5 = 250_000.00;

	let sales1 = qty1 as f64 * amt1;
	let sales2 = qty2 as f64 * amt2;
	let sales3 = qty3 as f64 * amt3;
	let sales4 = qty4 as f64 * amt4;
	let sales5 = qty5 as f64 * amt5;

	let sum = sales1 + sales2 + sales3 + sales4 +sales5;
	println!("Sum is {}", sum);

	let avg = sum / 5.00;
	println!("Average is {}", avg);
}
