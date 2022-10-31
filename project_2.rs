fn main() {

	//Toshiba
	let amount:f64 = 450000.0;
	let qty:f64 = 2.0;

	//Mac
	let amountmac:f64 = 1500000.0;
	let qtymac:f64 = 1.0;

	//HP
	let amounthp:f64 = 750000.0;
	let qtyhp:f64 = 3.0;

	//Dell
	let amountdell:f64 = 2850000.0;
	let qtydell:f64 = 3.0;

	//Acer
	let amountacer:f64 = 250000.0;
	let qtyacer:f64 = 1.0;

	let sum = (amount * qty) + (amountmac * qtymac) + (amounthp * qtyhp) + (amountdell * qtydell) + (amountacer + qtyacer);
	let qtysum = qty + qtymac + qtydell + qtyhp + qtyacer;
	let average = sum / qtysum;

	println!("Sum of the amounts is = {}", sum );
	println!("Average of the following sales records is {}", average );
}