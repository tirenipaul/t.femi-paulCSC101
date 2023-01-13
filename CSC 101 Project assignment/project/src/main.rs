use std::io::Write;
use std::io;

fn code_7() {

    let mut file = std::fs::File::create("aigbona_juliet.txt").expect("create failed");
    file.write_all("DEPARTMENT: Consulting\nServices include: \n"
        .as_bytes()).expect("write failed");
    let services = ["Analytics consulting services\n", "Customer experience\n", "Cybersecurity, strategy, risk, compliance and resilience\n", "Digital transformation\n", "Risk consulting services\n", "Supply chain and operations\n", "Technology transformation"];
    for i in 0..7{
    file.write_all(services[i].as_bytes()).expect("write failed");
    }

    let mut file = std::fs::File::create("akpevwe_iloka.txt").expect("create failed");
    file.write_all("DEPARTMENT: Assurance\nServices include: \n"
        .as_bytes()).expect("write failed");
    let services2 = ["Audit services\n", "Climate change and sustainability services\n", "Financial accounting advisory services\n", "Forensic and integrity services\n", "Private client audit experience\n", "Accounting Link\n", "Assurance"];
    for i in 0..7{
    file.write_all(services2[i].as_bytes()).expect("write failed");
    }

    println!("Data written to file");
}

fn code_8() {
    let mut file = std::fs::File::create("adamu_sagamu.txt").expect("write failed");
    file.write_all("DEPARTMENT: Tax\nServices include: \n"
        .as_bytes()).expect("write failed");
    let services = ["Tax planning\n", "Tax function operations\n", "Tax policy and controversy\n", "Global trade\n", "Tax accounting\n", "Tax compliance\n", "Transaction tax"];
    for i in 0..7{
    file.write_all(services[i].as_bytes()).expect("write failed");
    }

    let mut file = std::fs::File::create("gbenga_daniels.txt").expect("write failed");
    file.write_all("DEPARTMENT: People and workforce\nServices include: \n"
        .as_bytes()).expect("write failed");
    let services2 = ["Change management and experience\n", "HR transformation\n", "Integrated workforce mobility\n", "Learning and development consulting\n", "Recognition and reward advisory\n", "Workforce analytics\n", "People and workforce"];
    for i in 0..7{
    file.write_all(services2[i].as_bytes()).expect("write failed");
    }

    println!("Data written to file");
}

fn code_9() {

    let mut file = std::fs::File::create("ehis_ero.txt").expect("write failed");
    file.write_all("DEPARTMENT: Strategy\nServices include: \n"
        .as_bytes()).expect("write failed");
    let services = ["Strategy consulting\n", "Corporate and growth strategy\n", "Transaction strategy and execution\n", "Restructruing and turnaround strategy\n", "Industry strategy\n", "Digital business builiding\n", "Commercial strategy"];
    for i in 0..7{
    file.write_all(services[i].as_bytes()).expect("write failed")
    }

    let mut file = std::fs::File::create("maria_akinsola.txt").expect("write failed");
    file.write_all("DEPARTMENT: Transactions and corporate finance\nServices include: \n"
        .as_bytes()).expect("write failed");
    let services2 = ["Corporate finance\n", "Divestments and carve-outs\n", "Sustainability and ESG Services\n", "M&A advisory\n", "M&A integration\n", "M&A technology and tools\n", "M&A advanced analytics"];
    for i in 0..7{
    file.write_all(services2[i].as_bytes()).expect("write failed")
    }

    println!("Data written to file");
}
fn main() {

    println!("What code function do you want to perform?\n");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let code_choice:i64 = input1.trim().parse().expect("Invalid input");

    if code_choice == 7 {
        code_7();
    } 
    else if code_choice == 8 {
        code_8();
    }
    else if code_choice == 9 {
        code_9();
    }
}
