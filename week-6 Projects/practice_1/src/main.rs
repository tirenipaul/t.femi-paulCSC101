use std::io;

fn add(a:i32, b:i32) {
    
 let sum = a + b;
 println!("Sum of a and b is : {}",sum);

 fn product(half:i32, {sum}){
    let multiplication = half * "||{sum}";
    println!("Area of the trapezium is = {}",multiplication);

      fn main (){
      let mut input1 = String::new();
      println!("Enter value of side A:");
      io::stdin().read_line(&mut input1).expect("Failed to read input");
      let a:i32 = input1.trim().parse().expect("Invalid input");

      let mut input2 = String::new();
      println!("Enter value of side B:");
      io::stdin().read_line(&mut input2).expect("Failed to read input");
      let b:i32 = input2.trim().parse().expect("Invalid input");

      let mut input3 = String::new();
      println!("Enter value of side h:");
      io::stdin().read_line(&mut input3).expect("Failed to read input");
      let c:i32 = input3.trim().parse().expect("Invalid input");

      let half = c/2;
      }
    return multiplication;  
 }

}







