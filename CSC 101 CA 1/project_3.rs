fn main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// compound interest
	let i =  1.0 - ( 5.0 / 100.0 );
	let ei = i * i * i;
	let a = p * ei;

	println!("Depreciation of the TV after 3 years is {}", a );
}