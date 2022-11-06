fn main() {
    //addition
    let sum = 5500 + 7310;
    println!("The sum of 5500 and 7310 = {}",sum);

    //subtraction
    let difference:f32 = 95.5 - 4.3;  //error corrected (should be f32 instead of u32 as the difference doesn't result in a negative answer and is a decimal)
    println!("The difference of 95.5 and 4.3 = {}",difference);

    //multiplication
    let product = 4 * 30;  //error corrected (should be an integer instead of f:32)
    println!("The multiple of 4 and 30 = {}",product);

    //division
    let quotient:f64 = 56.7 / 32.2;
    println!("The division of 56.7 and 32.2 = {}",quotient);

    //remainder
    let remainder = 43 % 5;
    println!("The remainder of 43 and 5 is {}",remainder);
}
