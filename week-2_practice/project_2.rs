fn main(){

	let amountt:f64 = 450000.0;
	let qtyt:f64 = 2.0;
	let tott:f64 = qtyt * amountt;

	let amountm:f64 = 1500000.0;
	let qtym:f64 = 1.0;
	let totm:f64 = qtym * amountm;

	let amounthp:f64 = 750000.0;
	let qtyhp:f64 = 3.0;
	let tothp:f64 = qtyhp * amounthp;

	let amountdell:f64 = 2850000.0;
	let qtydell:f64 = 3.0;
	let totdell:f64 = qtydell * amountdell;

	let amountacer:f64 = 250000.0;
	let qtyacer:f64 = 1.0;
	let totacer:f64 = qtyacer * amountacer;

	let total_sum = totacer + totm + tott + tothp + totdell;
	let average = total_sum / 5.0;

	println!("The sum of the sales is {}", total_sum);
	println!("The average of the sales is {}", average);

}