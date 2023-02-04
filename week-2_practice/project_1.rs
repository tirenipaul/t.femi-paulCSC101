fn main(){
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	let x = 1.0 + (r / 100.0);
	let exp = x * x * x * x * x;
	let a = p * exp;
	let ci = a - p;

    println!("The amount after 5 years is {}", a);
	println!("The compound interest for 5 years at 10% per annum compounded annually is {}", ci); 
}