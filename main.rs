use std::io;

fn main() {
    println!("How many number of siblings do you have?:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid string");
    let mut number = input.trim().parse().expect("Invalide number");

    let mut siblings = vec![String::new(); number];
    for x in 0..number{
        println!("\nEnter first name of sibling: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        siblings [x] = name.trim().into();

        println!("Enter age of this sibling: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read string");
        let age:i32 = input1.trim().parse().expect("Failed to read number");

        if age > 18 {
            println!("Is this sibling married or single?");
            println!("Take 1 for married and 0 for single. Answer: ");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("Failed to read string");
            let marital:i32 = input2.trim().parse().expect("Failed to read number");

            if marital == 0{
                println!("Is this sibling a student or a worker?");
                println!("Take 1 for student and 2 for worker. Answer: ");
                let mut input3 = String::new();
                io::stdin().read_line(&mut input3).expect("Failed to read string");
                let mut prof:i32 = input3.trim().parse().expect("Failed to read number");

                if prof == 1{
                    println!("What University does this sibling attend?: ");
                    let mut uni = String::new();
                    io::stdin().read_line(&mut uni).expect("Invalid string");

                    println!("What course is this sibling studying?: ");
                    let mut course = String::new();
                    io::stdin().read_line(&mut course).expect("Invalid string");
                }
            }
            else if marital == 1{
                println!("Does this sibling have any offspring?: ");
                let mut child = String::new();
                io::stdin().read_line(&mut child).expect("Invalid string");

                println!("What city does this sibling's family live in?: ");
                let mut city = String::new();
                io::stdin().read_line(&mut city).expect("Invalid string");
            }
        }
        else if age < 18{
            println!("Has this sibling written WAEC?: ");
            println!("Take 1 for yes and 0 for no. Answer: ");
            let mut input4 = String::new();
            io::stdin().read_line(&mut input4).expect("Failed to read string");
            let mut waec:i32 = input4.trim().parse().expect("Failed to read number");

            if waec == 1{
                println!("Kindly input the name of the Secondary School this sibling attended: ");
                let mut school = String::new();
                io::stdin().read_line(&mut school).expect("Invalid string");
            }
            else if waec == 0{
                println!("Kindly input the current class level this sibling is at: ");
                let mut class = String::new();
                io::stdin().read_line(&mut class).expect("Invalid string");
            }
        }
    }
    println!("\nData of all siblings is {:?}", siblings);

}
