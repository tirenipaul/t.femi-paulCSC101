use std::io::Read;
use std::io;

fn administration(){
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn project(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn staff(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn customer(){
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn main(){
    println!("What is your position in Management?\n");
    println!("Take 1 to be Administrator, 2 to be Project Manager, 3 to Employee, 4 to be Customer and 5 to be Vendor\n");
    println!("Enter you Management number: ");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let management_choice:i64 = input1.trim().parse().expect("Invalid input");

    if management_choice == 1 {
        administration();
    }
    else if management_choice == 2 {
        project();
    }
    else if management_choice == 3 {
        staff();
    }
    else {
        customer();
    }
}

