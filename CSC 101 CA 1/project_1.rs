fn main() {
	let p:f64 = 520_000000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	//compound interest
	let i = 1.0 + (r / 100.0);
	let ei = i * i * i * i *i;
	let a = p * ei; 
	println!("Amount is {}", a);
	let ci = a - p; 
	println!("Compound Interest is {}", ci);
}