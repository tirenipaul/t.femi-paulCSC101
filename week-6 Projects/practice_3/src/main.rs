use std::io;

fn trapezium(a:i32, b:i32, c:i32){
    println!("a is base 1, b is base 2 and c is height");
    let area = (c/2) * ( a + b);
    println!("The area of the trapezium is {}", area);
}

fn rhombus(a:i32, b:i32){
    println!("a is diagonal 1 and b is diagonal 2");
    let area = (1/2) * a * b;
    println!("The area of the rhombus is {}", area);
}

fn parallelogram(a:i32, b:i32){
    println!("a is the base and b is the altitude");
    let area = a * b;
    println!("The area of the parallelogram is {}", area);
}

fn cube(a:i32){
    let area = 6 * a * a;
    println!("The area of the cube is {}", area);
}

fn cylinder(a:i32, b:i32){
    println!("a is the radius and b is the height");
    let volume = (22/7) * a * a * b;
    println!("The volume of the cylinder is {}", volume);
}


fn main() {

    println!("S/N");
    println!("(1) Area of a Trapezium formula = height/2*(base1+base2)");
    println!("(2) Area of the Rhombus formula = 1/2 x diagonal1 x diagonal2");
    println!("(3) Area of Parallelogram formula = base x altitude");
    println!("(4) Area of Cube formula = 6 x (length of the side)^2");
    println!("(5) Volume of Cylinder formula");

    let mut inputx = String::new();
    println!("Enter S/N of chosen calculation");
    io::stdin().read_line(&mut inputx).expect("Failed to read input");
    let function:f64 = inputx.trim().parse().expect("Not a valid S/N");

    let mut input1 = String::new();
    println!("Enter input for parameter a");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input parameter for b");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter input value for parameter c");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:i32 = input3.trim().parse().expect("Invalid input");

    if function == 1.0 {
        trapezium(a,b,c);
    }
    else if function == 2.0 {
        rhombus(a,b);
    }
    else if function == 3.0 {
        parallelogram(a,b);
    }
    else if function == 4.0 {
        cube(a);
    }
    else {
        cylinder(a,b);
    }
}
