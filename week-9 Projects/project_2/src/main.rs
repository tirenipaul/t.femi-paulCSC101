use std::io::Write;

fn main() {
    let row1 = ["STUDENT NAME: ", "\nMATRIC. NUMBER:", "\nDEPARTMENT: ", "\nLEVEL: "];
    let row2 = ["Oluchi Mordi    ", "ACC10211111 ", "    Accounting  ", "             300"];
    let row3 = ["Adams Aliyu    ", "   ECO10110101 ", "   Economics ", "           100     "];
    let row4 = ["Shania Bolade   ", "    CSC10328828 ", "       Computer  ", "         200     "];
    let row5 = ["Adekunle Gold    ", "    EEE11020202", "    Electrical  ", "       200      "];
    let row6 = ["Blanca Edemoh    ", "      MEE10202001", "      Mechanical", "        100"];
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("PAU SIMS\n"
        .as_bytes()).expect("write failed");
    for i in 0..4{
        file.write_all(row1[i].as_bytes()).expect("write failed");
        file.write_all(row2[i].as_bytes()).expect("write failed");
        file.write_all(row3[i].as_bytes()).expect("write failed");
        file.write_all(row4[i].as_bytes()).expect("write failed");
        file.write_all(row5[i].as_bytes()).expect("write failed");
        file.write_all(row6[i].as_bytes()).expect("write failed");
    }
    println!("\nData written to file");
    
}
