use std::io::Write;

fn main() {

    let row1 = ["Larger: ", "\nStout: ", "\nNon-Alcoholic: "];
    let row2 = ["33 Export, ", "Legend, ", "Maltina, "];
    let row3 = ["Desperados, ", "Turbo King, ", "Amstel Malta, "];
    let row4 = ["Goldberg, ", "Wiliams", "Malta Gold, "];
    let row5 = ["Gulder, ", "", "Fayrouz"];
    let row6 = ["Heineken, ", "", ""];
    let row7 = ["Star, ", "", ""];
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("NIGERIAN BREWERY LIMITED\n"
        .as_bytes()).expect("write failed");
    for i in 0..3{
    file.write_all(row1[i].as_bytes()).expect("write failed");
    file.write_all(row2[i].as_bytes()).expect("write failed");
    file.write_all(row3[i].as_bytes()).expect("write failed");
    file.write_all(row4[i].as_bytes()).expect("write failed");
    file.write_all(row5[i].as_bytes()).expect("write failed");
    file.write_all(row6[i].as_bytes()).expect("write failed");
    file.write_all(row7[i].as_bytes()).expect("write failed");
    }
        println!("\nData written to file");
}
