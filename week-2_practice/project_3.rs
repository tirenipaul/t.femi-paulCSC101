fn main(){

	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	let x = 1.0 - (r / 100.0);
	let exp = x * x * x;
	let a = p * exp;
	let ci = p - a;

	println!("The value of the TV after 3 years is {}", a);
	println!("The depreciation is {}", ci);
} 